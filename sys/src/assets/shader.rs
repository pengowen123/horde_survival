//! Shader loading

use super::utils;

use std::path::Path;
use std::io;

pub fn load_shader_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    utils::read_bytes(path)
}
