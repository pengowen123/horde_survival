//! Functions that control entities using the AI

use world::Coords;
use items::WeaponType;
use entity::*;
use consts::ai_control::PROJECTILE_LEARNING_RATE;

/// Uses the AI to make decisions for an entity
/// Called every tick for AI controlled entities
// NOTE: Entities should only be controlled by one thing at a time, if it is used to control a
//       player entity, the player can move forward while the AI does, causing double speed, as
//       Entity::move_forward does not check for this
pub fn apply_ai(target_index: usize, entities: &mut Vec<Entity>) {
    let closest = get_closest_entity(target_index, &*entities);

    if let Some((i, distance)) = closest {
        let target_coords;
        let target_id;
        // Scoped for mutable borrow
        {
            let entity = &entities[i];
            target_coords = entity.coords;
            target_id = entity.id;
        }

        let entity = &mut entities[target_index];
        let range = entity.current_weapon.get_real_range();

        entity.direction = entity.coords.direction_to(&target_coords);

        if distance <= range {
            entity.attack = true;

            if let WeaponType::RangedProjectile = entity.current_weapon.weapon_type {
                if target_id != entity.ai_target_id {
                    entity.ai_projectile_error = 0.0;
                }

                entity.ai_target_id = target_id;
                entity.direction.0 = correct_for_error(entity.current_weapon.range,
                                                       entity.direction.0,
                                                       entity.ai_projectile_error);
            }
        }

        if distance >= range * RANGE_THRESHOLD {
            // If out of range, move forward
            entity.move_forward(0.0);
        } else if distance <= range * RANGE_TOO_CLOSE_THRESHOLD {
            // If too close, move backwards
            entity.move_forward(180.0);
        }
    }
}

/// Returns a corrected angle to shoot a projectile at, given the current angle, projectile speed,
/// and the error
pub fn correct_for_error(speed: f64, angle: f64, error: f64) -> f64 {
    let corrected = 4.0 + angle + error * PROJECTILE_LEARNING_RATE / speed;

    if corrected < 45.0 {
        45.0
    } else if corrected > 135.0 {
        135.0
    } else {
        corrected
    }
}

/// Calculates the projectile's error, given three coordinates
/// `a` is the location of the projectile's source entity
/// `b` is the location of the projectile when it landed
/// `c` is the location of the entity the AI intended to hit
pub fn calculate_error(a: &Coords, b: &Coords, c: &Coords) -> f64 {
    // Error is equal to the distance between points C and D, where D is the intersection of line AC
    // and its altitude

    let side_a = a.distance(b);
    let side_b = a.distance(c);
    let side_c = b.distance(c);

    let s = (side_a + side_b + side_c) / 2.0;
    let area = ((s - side_a) * (s - side_b) * (s - side_c)).sqrt();
    // height is the altitude of line AC
    let height = area / (side_a / 2.0);
    let side_d = (side_b.powi(2) - height.powi(2)).sqrt();

    // The error is negative if point D is farther than the target location
    if side_d - side_a > 0.0 {
        side_d
    } else {
        side_d * -1.0
    }
}
