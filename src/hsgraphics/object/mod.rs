//! Types for storing data to be drawn

pub mod object2d;
pub mod object3d;
pub mod object_gui;

use {gfx, gfx_device_gl};

/// An encoder from gfx
pub type ObjectEncoder = gfx::Encoder<gfx_device_gl::Resources, gfx_device_gl::CommandBuffer>;
