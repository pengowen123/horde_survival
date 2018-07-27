//! Shader loading and preprocessing

use regex::bytes::{Regex, Replacer, Captures};
use rendergraph::error::BuildError;

use std::path::{Path, PathBuf};
use std::io;
use std::string::FromUtf8Error;
use std::collections::HashMap;

use super::utils;

const MAX_RECURSION_DEPTH: usize = 32;

/// A wrapper for `std::io::Error` that includes the file path
#[derive(Debug)]
pub struct IoError(pub PathBuf, pub io::Error);

impl IoError {
    pub fn path(&self) -> &Path {
        self.0.as_path()
    }

    pub fn err(&self) -> &io::Error {
        &self.1
    }
}

quick_error! {
    /// An error while loading a shader from a file
    #[derive(Debug)]
    pub enum ShaderLoadingError {
        Io(err: IoError) {
            display("Io error while reading `{}`: {}",
                    err.path().to_str().expect("Path contained invalid UTF-8"), err.err())
            from()
        }
        MaxIncludeRecursion {
            display("Maximum recursion depth reached while processing `#include` directives")
        }
        Utf8(err: FromUtf8Error) {
            display("Invalid UTF-8 in shader name: {:?}", err)
            from()
        }
    }
}

impl From<ShaderLoadingError> for BuildError<String> {
    fn from(e: ShaderLoadingError) -> BuildError<String> {
        BuildError::Custom(e.into())
    }
}

/// Loads a shader from the file at the provided path
///
/// Applies special parsing such as processing `#include` directives and inserting `#define`s
pub fn load_shader_file<P: AsRef<Path>>(
    path: P,
    defines: &HashMap<String, String>,
) -> Result<Vec<u8>, ShaderLoadingError> {
    let mut result = load_shader_file_with_includes(&path.as_ref(), 0)?;

    // The index at which to insert the `#define` statements
    let defines_index = result
        .iter()
        .position(|byte| *byte == b'\n')
        .map(|p| p + 1)
        .unwrap_or_else(|| result.len());

    for (key, val) in defines {
        let define_statement = format!("#define {} {}", key, val).into_bytes();

        // Many characters will be inserted individually, so reserve space to avoid frequent
        // allocations
        result.reserve(define_statement.len());

        for c in define_statement.into_iter().rev() {
            result.insert(defines_index, c);
        }
    }

    Ok(result)
}

/// Loads the shader from the file at the provided path, and processes `#include` directives
///
/// `#define` processing happens in `load_shader_file` so that the recursive `#include` processing
/// can avoid applying `#define`s to `#include`ed shaders
fn load_shader_file_with_includes(
    path: &Path,
    recurses: usize
) -> Result<Vec<u8>, ShaderLoadingError> {
    if recurses > MAX_RECURSION_DEPTH {
        return Err(ShaderLoadingError::MaxIncludeRecursion);
    }

    let bytes = utils::read_bytes(path)
        .map_err(|err| {
            ShaderLoadingError::Io(IoError(path.to_owned(), err))
        })?;

    lazy_static! {
        static ref FIND_INCLUDE: Regex = Regex::new(r#"#include "(.*)""#).unwrap();
    }

    let mut replacer = IncludeReplacer::new(recurses);

    let result = FIND_INCLUDE.replace_all(&bytes, &mut replacer).into_owned();

    let _ = replacer.error?;

    Ok(result)
}

// NOTE: All these `recurses` variables are for tracking recursion depth so an error can be made if
//       it exceeds the limit

/// A type for replacing `#include` directives with the file contents at the path specified by the
/// directive
struct IncludeReplacer {
    error: Result<(), ShaderLoadingError>,
    recurses: usize,
}

impl IncludeReplacer {
    fn new(recurses: usize) -> Self {
        IncludeReplacer {
            error: Ok(()),
            recurses,
        }
    }
}

impl Replacer for IncludeReplacer {
    fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>) {
        let file_contents = match replace_include(caps, self.recurses) {
            Ok(bytes) => bytes,
            Err(e) => {
                self.error = Err(e);
                return;
            }
        };

        dst.extend(file_contents);
        dst.extend(b"\n");
    }
}

impl<'a> Replacer for &'a mut IncludeReplacer {
    fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>) {
        (**self).replace_append(caps, dst)
    }
}

/// The actual implementation of `replace_append` for `IncludeReplacer`, made separate for nicer
/// error handling
fn replace_include(caps: &Captures, recurses: usize) -> Result<Vec<u8>, ShaderLoadingError> {
    let name = caps[1].to_vec();
    let name = String::from_utf8(name).map_err(|e| ShaderLoadingError::Utf8(e))?;
    let path = super::get_shader_path(&name);

    load_shader_file_with_includes(&Path::new(&path), recurses + 1)
}
