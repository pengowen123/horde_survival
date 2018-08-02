//! Asset management

#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[macro_use]
extern crate slog;

pub mod shader;

use std::path::{PathBuf, Path};
use std::io::{self, Read};
use std::fs::File;

/// A type that calculates paths to assets based on the location of the assets directory
pub struct Assets {
    assets_dir: PathBuf,
    // These directories are calculated when the type is constructed
    shaders_dir: PathBuf,
    models_dir: PathBuf,
    fonts_dir: PathBuf,
}

impl Assets {
    pub fn new<P: Into<PathBuf>>(log: &slog::Logger, assets_dir: P)
        -> Result<Self, shader::IoError>
    {
        let assets_dir = assets_dir.into();
        let assets_dir = assets_dir
            .canonicalize()
            .map_err(|e| shader::IoError(assets_dir, e))?;
        let shaders_dir = assets_dir.join("shaders");
        let models_dir = assets_dir.join("models");
        let fonts_dir = assets_dir.join("fonts");

        info!(log, "Creating assets manager"; o!("assets_dir" => assets_dir.to_str().unwrap()));

        Ok(Self {
            assets_dir,
            shaders_dir,
            models_dir,
            fonts_dir,
        })
    }

    /// Returns the path to the assets directory
    pub fn get_assets_dir(&self) -> &Path {
        &self.assets_dir
    }

    /// Returns a path to a shader file given a path relative to the shaders directory
    pub fn get_shader_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.shaders_dir.join(path)
    }

    /// Returns a path to a model file given a path relative to the models directory
    pub fn get_model_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.models_dir.join(path)
    }

    /// Returns a path to a font file given a path relative to the fonts directory
    pub fn get_font_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.fonts_dir.join(path)
    }
}

/// Returns the bytes in the file at the provided path
pub fn read_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}
