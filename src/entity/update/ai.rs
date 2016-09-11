use world::direction::correct_for_error;
use items::WeaponType;
use player::Player;
use entity::*;
use consts::ai_control::*;

pub fn apply_ai(target_index: usize, entities: &mut Vec<Entity>, next_id: &mut usize, player: &mut Player) {
    let closest = get_closest_entity(target_index, &*entities);
    let id = entities[target_index].id;

    let mut attack = false;

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
            attack = true;

            if let WeaponType::RangedProjectile = entity.current_weapon.weapon_type {
                if target_id != entity.ai_target_id {
                    entity.ai_projectile_error = 0.0;
                }

                entity.ai_target_id = target_id;
                entity.direction.0 = correct_for_error(entity.direction.0, entity.ai_projectile_error);
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

    if attack {
        try_attack(id, entities, next_id, player);
    }
}
