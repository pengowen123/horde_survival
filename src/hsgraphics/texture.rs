use gfx::{self, tex};
use gfx_device_gl;

use consts::graphics::textures::*;
use hsgraphics::gfx3d::ColorFormat;

pub type Texture = gfx::handle::ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>;
pub type Texels = &'static [[u8; 4]; 1];

// TODO: Delete this
pub fn create_texture<F>(factory: &mut F, texels: &[&[[u8; 4]]]) -> Texture
    where F: gfx::Factory<gfx_device_gl::Resources>
{
    factory.create_texture_const::<ColorFormat>(tex::Kind::D2(1, 1, tex::AaMode::Single),
                                                       texels)
        .expect("Failed to create texture").1
}

pub use image_utils::{load_texture, load_texture_raw};

pub fn create_all_textures(factory: &mut gfx_device_gl::Factory) -> Vec<Texture> {
    vec![
        // NOTE: The order of these is important
        create_texture(factory, &[TEXELS_FLOOR]),
        create_texture(factory, &[TEXELS_PLAYER]),
        create_texture(factory, &[TEXELS_ZOMBIE]),
        create_texture(factory, &[TEXELS_FLYING_BALL_LINEAR]),
        create_texture(factory, &[TEXELS_FLYING_BALL_ARC]),
        create_texture(factory, &[TEXELS_MINIMAP_PLAYER]),
        create_texture(factory, &[TEXELS_MINIMAP_ZOMBIE]),
        create_texture(factory, &[TEXELS_MINIMAP_FLYING_BALL_LINEAR]),
        create_texture(factory, &[TEXELS_MINIMAP_FLYING_BALL_ARC]),
        create_texture(factory, &[TEXELS_BLUE]),
        create_texture(factory, &[TEXELS_BLACK]),
        create_texture(factory, &[TEXELS_GREEN]),
        create_texture(factory, &[TEXELS_CROSSHAIR]),
        load_texture(factory, &include_bytes!("../include/pepe.png")[..]),
    ]
}
