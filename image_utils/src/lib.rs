//! Utilities for creating `gfx` textures from images

// NOTE: This crate is standalone for compile time reasons

extern crate common;
extern crate image;
#[macro_use]
extern crate quick_error;

use common::gfx;

use gfx::handle::ShaderResourceView;
use gfx::{format, texture};

use std::io::Cursor;

pub use format::{Rgba8, Srgba8};
pub use image::{ImageError, JPEG, PNG};

/// Loads a texture from the provided data
pub fn load_texture<F, R, CF>(
    factory: &mut F,
    data: &[u8],
    format: image::ImageFormat,
) -> Result<ShaderResourceView<R, CF::View>, TextureError>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    CF: format::Formatted,
    CF::Channel: format::TextureChannel,
    CF::Surface: format::TextureSurface,
{
    let img = image::load(Cursor::new(data), format)?.to_rgba();
    let (w, h) = img.dimensions();
    load_texture_raw::<_, _, CF>(factory, [w, h], &img.into_vec()).map_err(|e| e.into())
}

/// Loads a texture from the data, assuming it is the given size and in the format specified by
/// `CF`
pub fn load_texture_raw<F, R, CF>(
    factory: &mut F,
    size: [u32; 2],
    data: &[u8],
) -> Result<ShaderResourceView<R, CF::View>, gfx::CombinedError>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    CF: format::Formatted,
    CF::Channel: format::TextureChannel,
    CF::Surface: format::TextureSurface,
{
    let kind = texture::Kind::D2(
        size[0] as texture::Size,
        size[1] as texture::Size,
        texture::AaMode::Single,
    );

    factory
        .create_texture_immutable_u8::<CF>(kind, texture::Mipmap::Allocated, &[data])
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
pub fn load_cubemap<F, R, CF>(
    factory: &mut F,
    data: CubemapData,
    format: image::ImageFormat,
) -> Result<gfx::handle::ShaderResourceView<R, CF::View>, TextureError>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    CF: format::Formatted,
    CF::Channel: format::TextureChannel,
    CF::Surface: format::TextureSurface,
{
    let images = data
        .as_array()
        .into_iter()
        .map(|d| image::load(Cursor::new(d), format).map(|i| i.to_rgba()))
        .collect::<Result<Vec<_>, _>>()?;

    let data: [&[u8]; 6] = [
        &images[0], &images[1], &images[2], &images[3], &images[4], &images[5],
    ];

    let kind = texture::Kind::Cube(images[0].dimensions().0 as u16);

    let texture =
        factory.create_texture_immutable_u8::<CF>(kind, texture::Mipmap::Allocated, &data[..])?;

    Ok(texture.1)
}

quick_error! {
    #[derive(Debug)]
    pub enum TextureError {
        // An error occured while loading an image
        Image(err: image::ImageError) {
            display("Image error: {}", err)
            from()
        }
        // An error occured while creating a texture
        Creation(err: gfx::CombinedError) {
            display("Texture creation error: {}", err)
            from()
        }
    }
}
