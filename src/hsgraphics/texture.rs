use gfx::{Factory, Resources, handle, tex};
use gfx::handle::ShaderResourceView;
use gfx_device_gl;

pub type Texture = ShaderResourceView<gfx_device_gl::Resources, [f32; 4]>;

pub use image_utils::{load_texture, load_texture_raw};

use hsgraphics::{ColorFormat, SurfaceFormat};

pub fn create_cache_texture<F, R>(factory: &mut F, size: usize) -> (handle::Texture<R, SurfaceFormat>, ShaderResourceView<R, [f32; 4]>)
    where F: Factory<R>, R: Resources
{
    let data = vec![0; size * size * 4];

    let kind = tex::Kind::D2(size as tex::Size, size as tex::Size, tex::AaMode::Single);

    match factory.create_texture_const_u8::<ColorFormat>(kind, &[&data]) {
        Ok(t) => t,
        Err(e) => crash!("Failed to create texture: {}", e),
    }
}
