//! Asset management, loads and stores assets

// TODO: Replace this module with something from crates.io

pub mod load;
pub mod single;
#[macro_use]
mod utils;

pub use self::load::*;
pub use self::single::*;

use gfx_device_gl::Factory;

use hsgraphics::texture::Texture;

use std::path::Path;
use std::collections::HashMap;
use std::io::{self, Error, ErrorKind};

/// Manages all assets
pub struct AssetLoader<P: AsRef<Path>> {
    textures: HashMap<&'static str, Asset<Texture, P>>,
}

impl<P: AsRef<Path>> Default for AssetLoader<P> {
    fn default() -> Self {
        AssetLoader { textures: HashMap::new() }
    }
}

impl_asset_methods!(AssetLoader,
                    Texture,
                    textures,
                    get_texture,
                    load_texture,
                    get_or_load_texture,
                    add_texture_assets,
                    "Texture");
