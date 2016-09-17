pub mod object2d;
pub mod object3d;

use {gfx, gfx_device_gl};

use hsgraphics::gfx2d;

pub type ObjectEncoder = gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>;
pub type ObjectColor = gfx::handle::RenderTargetView<gfx_device_gl::Resources, gfx2d::ColorFormat>;
pub type ObjectDepth = (gfx::format::D24_S8, gfx::format::Unorm);
