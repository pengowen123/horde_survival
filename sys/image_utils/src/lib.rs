extern crate image;
extern crate gfx;
#[macro_use]
extern crate quick_error;

use gfx::{texture, format};
use gfx::handle::ShaderResourceView;

use std::io::Cursor;

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

/// Loads a texture from the data, assuming it is the given size and in RGBA format
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
        .create_texture_immutable_u8::<format::Srgba8>(kind, &[data])
        .map(|(_, view)| view)
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
