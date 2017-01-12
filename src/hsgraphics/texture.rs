use gfx::*;
use gfx::handle::ShaderResourceView;
use gfx_device_gl;

/// A texture
pub type Texture = ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>;

pub use image_utils::{load_texture, load_texture_raw};

use hsgraphics::{ColorFormat, SurfaceFormat, FullFormat, ObjectEncoder};

/// Puts the data into the texture
pub fn update_texture(encoder: &mut ObjectEncoder,
                      texture: &handle::Texture<gfx_device_gl::Resources, SurfaceFormat>,
                      offset: [u16; 2],
                      size: [u16; 2],
                      data: &[[u8; 4]]) {

    // Info about the data
    let info = tex::ImageInfoCommon {
        xoffset: offset[0],
        yoffset: offset[1],
        zoffset: 0,
        width: size[0],
        height: size[1],
        depth: 0,
        format: (),
        mipmap: 0,
    };

    // Update the texture
    encoder.update_texture::<SurfaceFormat, FullFormat>(texture, None, info, data)
        .unwrap_or_else(|e| {
            crash!("Failed to update texture: {:?}", e);
        })
}

/// Creates and returns a texture with the given dimensions
pub fn create_texture<F, R>
    (factory: &mut F,
     width: u32,
     height: u32)
     -> (handle::Texture<R, SurfaceFormat>, ShaderResourceView<R, [f32; 4]>)
    where F: Factory<R>,
          R: Resources
{
    // Initialize the texture data (multiplied by 4 because each pixel is 4 bytes)
    let data = vec![0; (width * height * 4) as usize];

    // Texture kind
    let kind = tex::Kind::D2(width as tex::Size, height as tex::Size, tex::AaMode::Single);

    // Create the texture
    factory.create_texture_const_u8::<ColorFormat>(kind, &[&data])
        .unwrap_or_else(|e| crash!("Failed to create texture: {}", e))
}
