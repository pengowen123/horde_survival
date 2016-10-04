#[macro_use]
pub mod shapes;
pub mod state;
pub mod gfx3d;
pub mod object;
pub mod gfx2d;
pub mod texture;
pub mod camera;
pub mod options;
mod minimap;
mod entity;
mod utils;
mod init;
mod text;

pub use self::state::*;
pub use self::object::*;
pub use self::shapes::*;
pub use self::gfx2d::CLEAR_COLOR;
pub use self::texture::*;
pub use self::camera::*;
pub use self::options::*;

use gfx::format;

pub type ColorFormat = format::Srgba8;
