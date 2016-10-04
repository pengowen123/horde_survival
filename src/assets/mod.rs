pub mod load;
pub mod single;
#[macro_use]
mod utils;

pub use self::load::*;
pub use self::single::*;

use gfx_device_gl::Factory;
use rusttype::Font;

use hsgraphics::texture::Texture;

use std::path::Path;
use std::collections::HashMap;
use std::io::{self, Error, ErrorKind};

pub struct AssetLoader<P: AsRef<Path>> {
    pub font: Asset<Font<'static>, P>,
    textures: HashMap<&'static str, Asset<Texture, P>>,
}

impl<P: AsRef<Path>> AssetLoader<P> {
    pub fn new<S: Into<P>>(font_path: S) -> Self {
        AssetLoader {
            textures: HashMap::new(),
            font: Asset::new(font_path.into()),
        }
    }

    pub fn load_font(&mut self, factory: &mut Factory) -> io::Result<()> {
        self.font.load(factory)
    }
}

impl_asset_methods!(AssetLoader, Texture,
                    textures, load_texture, get_or_load_texture, add_texture_assets,
                    "Texture");
