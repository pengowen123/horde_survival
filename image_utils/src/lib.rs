extern crate image;
extern crate gfx_device_gl;
extern crate gfx;

use gfx::{Factory, tex, format};
use gfx::handle::ShaderResourceView;

use std::io::Cursor;

pub fn load_texture(factory: &mut gfx_device_gl::Factory, data: &[u8]) -> ShaderResourceView<gfx_device_gl::Resources, [f32; 4]> {
    let img = image::load(Cursor::new(data), image::PNG).expect("Failed to create texture").to_rgba();
    let (w, h) = img.dimensions();
    load_texture_raw(factory, [w, h], &img.into_vec())
}

pub fn load_texture_raw(factory: &mut gfx_device_gl::Factory, size: [u32; 2], data: &[u8]) -> ShaderResourceView<gfx_device_gl::Resources, [f32; 4]> {
    let kind = tex::Kind::D2(size[0] as tex::Size, size[1] as tex::Size, tex::AaMode::Single);
    let (_, view) = factory.create_texture_const_u8::<format::Srgba8>(kind, &[data])
        .expect("Failed to create raw texture");
    view
}
