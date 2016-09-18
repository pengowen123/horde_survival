use consts::*;
use entity::EntityType;
use world::Coords;

pub fn get_entity_box_size(entity_type: &EntityType) -> f32 {
    match *entity_type {
        EntityType::Player => 0.3,
        EntityType::Zombie => 0.3,
        EntityType::FlyingBallLinear => 0.075,
        EntityType::FlyingBallArc => 0.075,
    }
}

pub fn get_texture_id(entity_type: &EntityType) -> usize {
    match *entity_type {
        EntityType::Player => 1,
        EntityType::Zombie => 2,
        EntityType::FlyingBallLinear => 3,
        EntityType::FlyingBallArc => 4,
    }
}

pub fn get_scales(d: f32) -> (f32, f32) {
    (d * MINIMAP_SCALE / WINDOW_WIDTH as f32,
     d * MINIMAP_SCALE / WINDOW_HEIGHT as f32)
}

pub fn get_unscaled_cube_coords(coords: &Coords, cube_size: f32) -> [f32; 3] {
    let height = coords.y as f32 + cube_size + FLOOR_HEIGHT;
    [coords.x as f32, coords.z as f32, height]
}
