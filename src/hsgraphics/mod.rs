//! Everything related to drawing the game

#[macro_use]
pub mod shapes;
pub mod state;
pub mod gfx3d;
pub mod gfx2d;
pub mod gfx_gui;
pub mod object;
pub mod texture;
pub mod camera;
pub mod options;
pub mod cache;
mod shaders;
mod entity;
mod utils;
mod init;

pub use self::state::*;
pub use self::object::*;
pub use self::shapes::*;
pub use self::gfx2d::CLEAR_COLOR;
pub use self::texture::*;
pub use self::camera::*;
pub use self::options::*;
pub use self::cache::*;

use gfx::format;

pub type ColorFormat = format::Srgba8;
pub type SurfaceFormat = format::R8_G8_B8_A8;
pub type FullFormat = (SurfaceFormat, format::Unorm);
