use gfx::{self, tex};
use gfx_device_gl::Resources;

use hsgraphics::object::object3d::ShaderView;

pub type Texels = &'static [[u8; 4]; 1];

pub fn create_texture<F>(factory: &mut F, texels: &[&[[u8; 4]]]) -> ShaderView
    where F: gfx::Factory<Resources>
{
    factory.create_texture_const::<gfx::format::Rgba8>(tex::Kind::D2(1, 1, tex::AaMode::Single),
                                                       texels)
        .expect("Failed to create texture").1
}
