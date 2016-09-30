extern crate image;
extern crate gfx;
extern crate gfx_device_gl;

use gfx::tex;
use gfx::handle::ShaderResourceView;
use gfx_device_gl::Factory;

use std::io::Cursor;

pub fn load_texture(factory: &mut Factory, data: &[u8]) -> ShaderResourceView<gfx_device_gl::Resources, [f32; 4]> {
    let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
    let (w, h) = img.dimensions();
    load_texture_raw(factory, [w, h], &img.into_vec())
}

pub fn load_texture_raw<R, F>(factory: &mut F, size: [u32; 2], data: &[u8]) -> ShaderResourceView<R, [f32; 4]>
    where R: gfx::Resources, F: gfx::Factory<R>
{
    let kind = tex::Kind::D2(size[0] as tex::Size, size[1] as tex::Size, tex::AaMode::Single);
    let (_, view) = factory.create_texture_const_u8::<gfx::format::Rgba8>(kind, &[data]).unwrap();
    view
}
