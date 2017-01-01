use cgmath::Point3;
use collision::Aabb3;

use entity::{EntityType, Hitbox};
use world::Coords;

pub fn get_hitbox(entity_type: &EntityType, coords: &Coords) -> Hitbox {
    let (mut a, mut b) = match *entity_type {
        EntityType::Player | EntityType::Zombie => {
            (Point3::new(-0.12f64, -0.8, -0.12), Point3::new(0.12f64, 0.0, 0.12))
        }
        _ => (Point3::new(0f64, 0.0, 0.0), Point3::new(0f64, 0.0, 0.0)),
    };

    let coords = coords.as_vector();

    a += coords;
    b += coords;

    Aabb3::new(a, b)
}

pub fn get_entity_box_size(entity_type: &EntityType) -> f32 {
    match *entity_type {
        EntityType::Player | EntityType::Zombie => 0.1,
        EntityType::FlyingBallLinear |
        EntityType::FlyingBallArc => 0.05,
    }
}
