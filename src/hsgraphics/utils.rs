use entity::EntityType;
use world::Coords;

pub fn get_texture_name(entity_type: &EntityType) -> &str {
    match *entity_type {
        EntityType::Player => "player",
        EntityType::Zombie => "zombie",
        EntityType::FlyingBallLinear => "ball_linear",
        EntityType::FlyingBallArc => "ball_arc",
    }
}

pub fn get_unscaled_cube_coords(coords: &Coords) -> [f32; 3] {
    [coords.x as f32, coords.z as f32, coords.y as f32]
}
