pub mod window;
pub mod minimap;
pub mod textures;
mod camera;

pub use self::window::*;
pub use self::minimap::*;
pub use self::textures::*;
pub use self::camera::*;

pub const FLOOR_HEIGHT: f32 = -0.4;
pub const ENTITY_OBJECT_ID: usize = 1;
