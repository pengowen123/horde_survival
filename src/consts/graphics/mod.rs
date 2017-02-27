//! Constants involving graphics

pub mod window;
pub mod crosshair;
pub mod gui;
mod camera;

pub use self::window::*;
pub use self::camera::*;
pub use self::crosshair::*;
pub use self::gui::*;

use conrod::Color;

// Misc graphics constants

pub const FLOOR_HEIGHT: f32 = 0.4;
/// The change in position required to perform an update to an entity's model
pub const UPDATE_THRESHOLD: f64 = 0.05;

/// The ID used for each entity's object

// TODO: Replace object ids with an enum
//       Each variant could store an ID if necessary
pub const ENTITY_OBJECT_ID: usize = 1;

/// The color used when clearing the screen
// TODO: Replace this with [1.0; 4] (white) when a skybox is implemented
//       For now, this color is the sky
pub const CLEAR_COLOR: [f32; 4] = [0.0, 0.35, 0.5, 1.0];
