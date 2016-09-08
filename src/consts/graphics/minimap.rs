use hsgraphics::Color;

pub const MINIMAP_LOCATION: (f32, f32) = (0.0, 0.0);
pub const MINIMAP_SIZE: f32 = 200.0;
pub const MINIMAP_SCALE: f64 = 15.0 / MINIMAP_SIZE as f64;
// NOTE: Actualy entity size will much far smaller due to normalizing
pub const MINIMAP_ENTITY_SIZE: f32 = 20.0;

pub const MINIMAP_COLOR_PLAYER: Color = [0.0, 1.0, 0.0];
pub const MINIMAP_COLOR_ZOMBIE: Color = [1.0, 0.0, 0.0];
pub const MINIMAP_COLOR_FLYING_BALL_LINEAR: Color = [0.0, 0.0, 0.5];
pub const MINIMAP_COLOR_FLYING_BALL_ARC: Color = [0.0, 0.0, 1.0];

pub const MINIMAP_OBJECT_ID: usize = 0;
