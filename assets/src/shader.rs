//! Shader loading and preprocessing

use regex::bytes::{Regex, Replacer, Captures};

use std::path::{Path, PathBuf};
use std::{io, fmt};
use std::string::FromUtf8Error;
use std::collections::HashMap;

use Assets;
use read_bytes;

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

impl fmt::Display for IoError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt,
                 "Io error while reading `{}`: {}",
                 self.path().to_str().expect("Path contained invalid unicode"),
                 self.err())
    }
}

quick_error! {
    /// An error while loading a shader from a file
    #[derive(Debug)]
    pub enum ShaderLoadingError {
        Io(err: IoError) {
            display("{}", err)
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

/// Loads a shader from the file at the provided path
///
/// Applies special parsing such as processing `#include` directives and inserting `#define`s
pub fn load_shader_file<P: AsRef<Path>>(
    assets: &Assets,
    path: P,
    defines: &HashMap<String, String>,
) -> Result<Vec<u8>, ShaderLoadingError> {
    load_shader_file_impl(assets, path.as_ref(), defines)
}

pub fn load_shader_file_impl(
    assets: &Assets,
    path: &Path,
    defines: &HashMap<String, String>,
) -> Result<Vec<u8>, ShaderLoadingError> {
    let mut result = load_shader_file_with_includes(assets, path, 0)?;

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
        result.reserve(define_statement.len() + 1);

        result.insert(defines_index, b'\n');

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
    assets: &Assets,
    path: &Path,
    recurses: usize
) -> Result<Vec<u8>, ShaderLoadingError> {
    if recurses > MAX_RECURSION_DEPTH {
        return Err(ShaderLoadingError::MaxIncludeRecursion);
    }

    let path = assets.get_shader_path(path);
    let bytes = read_bytes(&path)
        .map_err(|err| {
            ShaderLoadingError::Io(IoError(path.to_owned(), err))
        })?;

    lazy_static! {
        static ref FIND_INCLUDE: Regex = Regex::new(r#"#include "(.*)""#).unwrap();
    }

    let mut replacer = IncludeReplacer::new(assets, recurses);

    let result = FIND_INCLUDE.replace_all(&bytes, &mut replacer).into_owned();

    let _ = replacer.error?;

    Ok(result)
}

// NOTE: All these `recurses` variables are for tracking recursion depth so an error can be made if
//       it exceeds the limit

/// A type for replacing `#include` directives with the file contents at the path specified by the
/// directive
struct IncludeReplacer<'a> {
    assets: &'a Assets,
    error: Result<(), ShaderLoadingError>,
    recurses: usize,
}

impl<'a> IncludeReplacer<'a> {
    fn new(assets: &'a Assets, recurses: usize) -> Self {
        IncludeReplacer {
            assets,
            error: Ok(()),
            recurses,
        }
    }
}

impl<'a> Replacer for IncludeReplacer<'a> {
    fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>) {
        let file_contents = match replace_include(self.assets, caps, self.recurses) {
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

impl<'a: 'b, 'b> Replacer for &'b mut IncludeReplacer<'a> {
    fn replace_append(&mut self, caps: &Captures, dst: &mut Vec<u8>) {
        (**self).replace_append(caps, dst)
    }
}

/// The actual implementation of `replace_append` for `IncludeReplacer`, made separate for nicer
/// error handling
fn replace_include(
    assets: &Assets,
    caps: &Captures,
    recurses: usize,
) -> Result<Vec<u8>, ShaderLoadingError> {
    let name = caps[1].to_vec();
    let name = String::from_utf8(name).map_err(|e| ShaderLoadingError::Utf8(e))?;

    load_shader_file_with_includes(assets, &Path::new(&name), recurses + 1)
}
