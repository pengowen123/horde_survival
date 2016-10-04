use gfx_device_gl::Factory;

use hsgraphics::texture::*;
use rusttype::{Font, FontCollection};

use std::io::{self, Read};
use std::fs::File;
use std::path::Path;

pub fn load_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = try!(File::open(path));
    let mut buf = Vec::new();

    try!(file.read_to_end(&mut buf));

    Ok(buf)
}

pub trait LoadAsset: Sized {
    fn load_asset<P: AsRef<Path>>(&mut Factory, P) -> io::Result<Self>;
}

impl LoadAsset for Texture {
    fn load_asset<P: AsRef<Path>>(factory: &mut Factory, path: P) -> io::Result<Self> {
        let bytes = try!(load_bytes(path));

        Ok(load_texture(factory, &bytes))
    }
}

impl LoadAsset for Font<'static> {
    fn load_asset<P: AsRef<Path>>(_: &mut Factory, path: P) -> io::Result<Self> {
        let bytes = try!(load_bytes(&path));

        match FontCollection::from_bytes(bytes).into_font() {
            Some(f) => Ok(f),
            None => crash!("Failed to create font from {}", path.as_ref().to_str().unwrap()),
        }
    }
}
