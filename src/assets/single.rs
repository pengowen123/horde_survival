use gfx_device_gl::Factory;

use assets::LoadAsset;

use std::path::Path;
use std::io;

pub struct Asset<T: LoadAsset, P: AsRef<Path>> {
    path: P,
    data: AssetData<T>,
}

pub enum AssetData<T: LoadAsset> {
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

    pub fn get(&self) -> Option<&T> {
        match self.data {
            AssetData::Loaded(ref a) => Some(a),
            AssetData::Unloaded => None,
        }
    }

    pub fn get_or_load(&mut self, factory: &mut Factory) -> io::Result<&T> {
        match self.data {
            AssetData::Loaded(ref a) => Ok(a),
            AssetData::Unloaded => {
                try!(self.load(factory));
                Ok(self.get().unwrap())
            },
        }
    }

    pub fn load(&mut self, factory: &mut Factory) -> io::Result<()> {
        let asset = try!(T::load_asset(factory, &self.path));

        self.data = AssetData::Loaded(asset);

        Ok(())
    }
}
