//! Types for graphics

use gfx::{self, format};

/// The color format for graphics
// FIXME: Manually gamma correct in shaders when glutin actually uses the provided color format
//        blocked on gfx/#1350
pub type ColorFormat = gfx::format::Srgba8;

/// The depth format for graphics
pub struct DepthFormat;

impl format::Formatted for DepthFormat {
    type Surface = format::D24_S8;
    type Channel = format::Unorm;
    type View = [f32; 4];

    fn get_format() -> format::Format {
        format::Format(format::SurfaceType::D24_S8, format::ChannelType::Unorm)
    }
}

/// A view into a texture
pub type TextureView<R> = gfx::handle::ShaderResourceView<R, [f32; 4]>;

/// A vertex buffer
pub type VertexBuffer<R, V> = gfx::handle::Buffer<R, V>;
