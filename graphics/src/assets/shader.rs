//! Shader loading and preprocessing

use regex::bytes::{Regex, Replacer, Captures};
use rendergraph::error::BuildError;

use std::path::Path;
use std::io;
use std::string::FromUtf8Error;

use super::utils;

const MAX_RECURSION_DEPTH: usize = 32;

/// A wrapper for `std::io::Error` that includes the file path
#[derive(Debug)]
pub struct IoError(String, io::Error);

quick_error! {
    /// An error while loading a shader from a file
    #[derive(Debug)]
    pub enum ShaderLoadingError {
        Io(err: IoError) {
            display("Io error while reading `{}`: {}", err.0, err.1)
            from()
        }
        MaxIncludeRecursion {
            display("Maximum recursion depth reached while processing `#include` directives")
        }
        Utf8(err: FromUtf8Error) {
            display("Shader name contained invalid Utf8: {}", err)
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
/// Applies special parsing such as processing `#include` directives
pub fn load_shader_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, ShaderLoadingError> {
    load_shader_file_impl(&path.as_ref(), 0)
}

fn load_shader_file_impl(path: &Path, recurses: usize) -> Result<Vec<u8>, ShaderLoadingError> {
    if recurses > MAX_RECURSION_DEPTH {
        return Err(ShaderLoadingError::MaxIncludeRecursion);
    }

    let bytes = utils::read_bytes(path)
        .map_err(|err| ShaderLoadingError::Io(IoError(path.to_str().unwrap().to_string(), err)))?;

    lazy_static! {
        static ref FIND_INCLUDE: Regex = Regex::new(r#"#include "(.*)""#).unwrap();
    }

    let mut replacer = IncludeReplacer::new(recurses);

    let result = FIND_INCLUDE.replace_all(&bytes, &mut replacer);

    let _ = replacer.error?;

    Ok(result.into_owned())
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
    let name = String::from_utf8(name)?;
    let path = super::get_shader_path(&name);

    load_shader_file_impl(&Path::new(&path), recurses + 1)
}