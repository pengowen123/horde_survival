use entity::EntityType;
use world::Coords;

pub fn get_texture_id(entity_type: &EntityType) -> usize {
    match *entity_type {
        EntityType::Player => 1,
        EntityType::Zombie => 2,
        EntityType::FlyingBallLinear => 3,
        EntityType::FlyingBallArc => 4,
    }
}

pub fn get_unscaled_cube_coords(coords: &Coords) -> [f32; 3] {
    [coords.x as f32, coords.z as f32, coords.y as f32]
}
