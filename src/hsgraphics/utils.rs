use entity::EntityType;
use world::Coords;

/// Returns the name of the texture used by entities of the given type

// NOTE: Maybe replace this with constants; either way these names must be maintained
pub fn get_texture_name(entity_type: &EntityType) -> &str {
    match *entity_type {
        EntityType::Player => "player",
        EntityType::Zombie => "zombie",
        EntityType::FlyingBallLinear => "ball_linear",
        EntityType::FlyingBallArc => "ball_arc",
    }
}

/// Converts Coords (used for the game) to an array (used for graphics)
pub fn get_unscaled_cube_coords(coords: &Coords) -> [f32; 3] {
    [coords.x as f32, coords.z as f32, coords.y as f32]
}
