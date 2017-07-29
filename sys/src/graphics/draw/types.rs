//! Type aliases for `gfx` types

use gfx;

use super::pipeline;

/// The color format for graphics
pub type ColorFormat = gfx::format::Srgba8;

/// The depth format for graphics
pub type DepthFormat = gfx::format::DepthStencil;

/// A render target view
pub type RenderTargetView<R> = gfx::handle::RenderTargetView<R, ColorFormat>;

/// A depth stencil view
pub type DepthTargetView<R> = gfx::handle::DepthStencilView<R, DepthFormat>;

/// A shader resource view
pub type ShaderResourceView<R> = gfx::handle::ShaderResourceView<R, ColorFormat>;

/// A view into a texture
pub type TextureView<R> = gfx::handle::ShaderResourceView<R, [f32; 4]>;

/// A vertex buffer
pub type VertexBuffer<R, V> = gfx::handle::Buffer<R, V>;
