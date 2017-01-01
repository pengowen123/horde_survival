use world::Coords;
use items::WeaponType;
use entity::*;
use consts::ai_control::*;

pub fn apply_ai(target_index: usize, entities: &mut Vec<Entity>) {
    let closest = get_closest_entity(target_index, &*entities);

    if let Some((i, distance)) = closest {
        let target_coords;
        let target_id;
        // Scoped for mutable borrow
        {
            let entity = &entities[i];
            target_coords = entity.coords.clone();
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

pub fn calculate_error(a: &Coords, b: &Coords, c: &Coords) -> f64 {
    let side_a = a.distance(b);
    let side_b = a.distance(c);
    let side_c = b.distance(c);

    let s = (side_a + side_b + side_c) / 2.0;
    let area = ((s - side_a) * (s - side_b) * (s - side_c)).sqrt();
    let height = area / (side_a / 2.0);
    let side_d = (side_b.powi(2) - height.powi(2)).sqrt();

    if side_d - side_a > 0.0 {
        side_d
    } else {
        side_d * -1.0
    }
}
