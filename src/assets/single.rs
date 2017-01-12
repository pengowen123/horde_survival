use gfx_device_gl::Factory;

use assets::LoadAsset;

use std::path::Path;
use std::io;

/// A single asset
pub struct Asset<T, P> {
    path: P,
    data: AssetData<T>,
}

/// Optional data for an asset
pub enum AssetData<T> {
    Loaded(T),
    Unloaded,
}

impl<T: LoadAsset, P: AsRef<Path>> Asset<T, P> {
    pub fn new(path: P) -> Self {
        Asset {
            path: path,
            data: AssetData::Unloaded,
        }
    }

    /// Returns the data of the asset, if it is loaded
    pub fn get(&self) -> Option<&T> {
        match self.data {
            AssetData::Loaded(ref a) => Some(a),
            AssetData::Unloaded => None,
        }
    }

    /// Like `Asset::get`, but loads the asset if it isn't loaded
    pub fn get_or_load(&mut self, factory: &mut Factory) -> io::Result<&T> {
        match self.data {
            AssetData::Loaded(ref a) => Ok(a),
            AssetData::Unloaded => {
                try!(self.load(factory));
                Ok(self.get().unwrap())
            }
        }
    }

    /// Load the asset
    pub fn load(&mut self, factory: &mut Factory) -> io::Result<()> {
        let asset = try!(T::load_asset(factory, &self.path));

        self.data = AssetData::Loaded(asset);

        Ok(())
    }
}
