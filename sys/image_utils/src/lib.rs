//! Utilities for creating `gfx` textures from images

// NOTE: This crate is standalone for compile time reasons

extern crate image;
extern crate gfx;
#[macro_use]
extern crate quick_error;

use gfx::{texture, format};
use gfx::handle::ShaderResourceView;

use std::io::Cursor;

/// The color format used by this library
pub type ColorFormat = format::Srgba8;

/// Loads a texture from the provided data
///
/// The data should be in the PNG format.
pub fn load_texture<F, R>(
    factory: &mut F,
    data: &[u8],
) -> Result<ShaderResourceView<R, [f32; 4]>, TextureError>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let img = image::load(Cursor::new(data), image::PNG)?.to_rgba();
    let (w, h) = img.dimensions();
    load_texture_raw(factory, [w, h], &img.into_vec()).map_err(|e| e.into())
}

/// Loads a texture from the data, assuming it is the given size and in the format specified by
/// `ColorFormat`
pub fn load_texture_raw<F, R>(
    factory: &mut F,
    size: [u32; 2],
    data: &[u8],
) -> Result<ShaderResourceView<R, [f32; 4]>, gfx::CombinedError>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let kind = texture::Kind::D2(
        size[0] as texture::Size,
        size[1] as texture::Size,
        texture::AaMode::Single,
    );

    factory
        .create_texture_immutable_u8::<ColorFormat>(kind, &[data])
        .map(|(_, view)| view)
}

pub struct CubemapData<'a> {
    pub right: &'a [u8],
    pub left: &'a [u8],
    pub up: &'a [u8],
    pub down: &'a [u8],
    pub back: &'a [u8],
    pub front: &'a [u8],
}

impl<'a> CubemapData<'a> {
    fn as_array(self) -> [&'a [u8]; 6] {
        [
            self.right.into(),
            self.left.into(),
            self.up.into(),
            self.down.into(),
            self.back.into(),
            self.front.into(),
        ]
    }
}

/// Loads a cubemap from the provided data
///
/// The data should be in the JPEG format.
pub fn load_cubemap<F, R>(
    factory: &mut F,
    data: CubemapData,
) -> Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, TextureError>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    let images = data.as_array()
        .into_iter()
        .map(|d| {
            image::load(Cursor::new(d), image::JPEG).map(|i| i.to_rgba())
        })
        .collect::<Result<Vec<_>, _>>()?;

    let data: [&[u8]; 6] = [
        &images[0],
        &images[1],
        &images[2],
        &images[3],
        &images[4],
        &images[5],
    ];

    let kind = texture::Kind::Cube(images[0].dimensions().0 as u16);

    let texture = factory.create_texture_immutable_u8::<ColorFormat>(
        kind,
        &data[..],
    )?;

    Ok(texture.1)
}

quick_error! {
    #[derive(Debug)]
    pub enum TextureError {
        /// An error occured while loading an image
        Image(err: image::ImageError) {
            display("Image error: {}", err)
            from()
        }
        /// An error occured while creating a texture
        Creation(err: gfx::CombinedError) {
            display("Texture creation error: {}", err)
            from()
        }
    }
}
