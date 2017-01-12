//! Functions for filtering and finding entities

// TODO: Implement a EntitySearch trait on KDTree<Entity> to avoid the iterator mess in
//       get_closest_entity and get_collided_entity

use collision::{Ray, Intersect};

use entity::{Entity, EntityType};
use world::Coords;

/// Filters entities, removing them if they are dead, or if their lifetime is over
// NOTE: If entities mysteriously disappear, or don't when they should, this is a good place to
//       start investigating
pub fn filter_entities(entities: &mut Vec<Entity>) {
    *entities = entities.iter()
        .cloned()
        .filter(|e| {
            let result = if e.entity_type == EntityType::Player || e.is_dummy() {
                true
            } else {
                !e.is_dead()
            };

            result && e.lifetime != 1
        })
        .collect();
}

/// Returns the index and distance of the closest enemy entity to the one at the given index, or None if
/// no other entities exist
pub fn get_closest_entity(index: usize, entities: &[Entity]) -> Option<(usize, f64)> {
    let entity = &entities[index];

    let mut closest_index = 0;
    let mut closest_distance = None;

    for (i, e) in entities.iter().enumerate() {
        // The is_enemy_of check excludes the same entity
        if e.is_dummy() || !e.is_enemy_of(entity) {
            continue;
        }

        let distance = e.coords.distance(&entity.coords);

        if let Some(ref mut d) = closest_distance {
            if distance < *d {
                *d = distance;
                closest_index = i;
            }
        } else {
            closest_distance = Some(distance);
            closest_index = i;
        }
    }

    if let Some(d) = closest_distance {
        Some((closest_index, d))
    } else {
        None
    }
}

/// Returns the index of the entity that a projectile has passed through, if any
pub fn get_collided_entity(projectile_index: usize,
                           entities: &[Entity],
                           old_pos: Coords,
                           new_pos: Coords)
                           -> Option<usize> {

    let distance = old_pos.distance(&new_pos);
    let diff = Coords::new(new_pos.x - old_pos.x,
                           new_pos.y - old_pos.y,
                           new_pos.z - old_pos.z)
        .into();
    let ray = Ray::new(old_pos.into(), diff);
    let entity = &entities[projectile_index];

    for (i, e) in entities.iter().enumerate() {
        if e.is_dummy() || !e.is_enemy_of(entity) {
            continue;
        }

        let intersection = (ray, e.hitbox).intersection();

        if let Some(p) = intersection {
            if old_pos.distance(&Coords::new(p.x, p.y, p.z)) <= distance {
                return Some(i);
            }
        }
    }

    None
}

macro_rules! find_player_entity {
    ($entities:expr, $player:ident) => {{
        $entities.find(|e| e.id == $player.entity_id).unwrap_or_else(|| {
            crash!("Player entity not found");
        })
    }};
    (INDEX, $entities:expr, $player:ident) => {{
        $entities
             .enumerate()
             .find(|&(_, ref e)| e.id == $player.entity_id)
             .unwrap_or_else(|| crash!("Player entity not found"))

    }};
}
