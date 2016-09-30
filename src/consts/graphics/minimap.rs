use consts::graphics::window::*;

pub const MINIMAP_LOCATION: (f32, f32) = (-0.8, -0.8);
pub const MINIMAP_SIZE: f32 = 200.0;
pub const MINIMAP_SCALE: f32 = 400.0 / MINIMAP_SIZE;
pub const MINIMAP_ENTITY_SIZE: f32 = 0.0035 * ((WINDOW_WIDTH + WINDOW_HEIGHT) as f32 / 2.0);

pub const MINIMAP_OBJECT_ID: usize = 0;
