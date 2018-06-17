//! Types for graphics

use gfx::{self, format, handle};
use rendergraph::builder;

/// A `GraphBuilder` with color and depth format types specific to this crate
pub type GraphBuilder<'a, R, C, F> =
    builder::GraphBuilder<
        'a,
        R,
        C,
        F,
        ColorFormat,
        DepthFormat
    >;

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

/// The aspect ratio of a render target
#[derive(Clone, Copy, Debug)]
pub struct AspectRatio(pub f32);

impl AspectRatio {
    pub fn from_render_target<R, CF>(rtv: &handle::RenderTargetView<R, CF>) -> Self
    where
        R: gfx::Resources,
    {
        let (width, height, _, _) = rtv.get_dimensions();

        AspectRatio(width as f32 / height as f32)
    }

    pub fn from_depth_stencil<R, DF>(dsv: &handle::DepthStencilView<R, DF>) -> Self
    where
        R: gfx::Resources,
    {
        let (width, height, _, _) = dsv.get_dimensions();

        AspectRatio(width as f32 / height as f32)
    }
}

impl Default for AspectRatio {
    fn default() -> Self {
        // NOTE: This cannot be 0.0 or a panic will happen when creating a projection matrix
        AspectRatio(1.0)
    }
}
