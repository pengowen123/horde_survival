pub mod window;
pub mod minimap;
pub mod textures;
pub mod crosshair;
mod camera;
mod gui;

pub use self::window::*;
pub use self::minimap::*;
pub use self::textures::*;
pub use self::camera::*;
pub use self::crosshair::*;
pub use self::gui::*;

pub const FLOOR_HEIGHT: f32 = 0.4;
pub const UPDATE_THRESHOLD: f64 = 0.05;
pub const ENTITY_OBJECT_ID: usize = 1;

pub const GUI_CLEAR_COLOR: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
