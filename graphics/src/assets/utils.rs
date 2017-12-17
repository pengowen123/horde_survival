//! Utilities for asset management

use std::path::Path;
use std::io::{self, Read};
use std::fs::File;

/// Returns the bytes in the file at the provided path
pub fn read_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}
