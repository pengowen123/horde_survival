use world::coords::calculate_error;
use consts::*;
use entity::*;
use player::*;
use world::*;
use map::*;
use hslog::CanUnwrap;

pub fn update_flying_ball_linear(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player) {
    let points;

    // Scoped for update_flying_ball call
    {
        let entity = &entities[target_index];
        points = (entity.as_mods[0].value * 10.0) as usize / 10 + 1;
    }

    for _ in 0..points {
        if update_flying_ball(target_index, entities, player) {
            break;
        }

        let entity = &mut entities[target_index];

        entity.coords.move_3d(entity.direction, RANGED_INTERVAL * RANGED_LINEAR_SPEED);
    }
}

pub fn update_flying_ball_arc(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player, map: &Map) {
    let points_float;
    let points;
    let on_ground;

    // Scoped for update_flying_ball call
    {
        let entity = &entities[target_index];
        on_ground = entity.on_ground;
        points_float = entity.velocity.component_x * 10.0;
        points = (points_float as usize) / 10 + 1;
    }

    if on_ground {
        let mut found_target = true;
        let spawned_by;
        let projectile_coords;
        let id;
        let target_coords;
        
        // Scoped for iter_mut call
        {
            let entity = &entities[target_index];
            spawned_by = entity.spawned_by;
            projectile_coords = entity.coords.clone();
            id = entity.id;

            let target = entities.iter().find(|e| e.id == entity.ai_target_id);

            match target {
                Some(e) => target_coords = e.coords.clone(),
                None => {
                    found_target = false;
                    target_coords = Coords::origin();
                },
            }
        }

        if found_target {
            let source_id = unwrap_or_log!(spawned_by, "Flying ball ID {} has no source", id);
            let source = entities.iter_mut().find(|e| e.id == source_id);

            if let Some(e) = source {
                if e.has_ai() {
                    let mut new_error = PROJECTILE_LEARNING_RATE * calculate_error(&e.coords, &projectile_coords, &target_coords);
                    new_error = new_error * (0.5 / PROJECTILE_LEARNING_RATE).powi(4);

                    let increase = if e.ai_projectile_error > 0.0 {
                        new_error - e.ai_projectile_error
                    } else {
                        e.ai_projectile_error - new_error
                    };

                    if increase.abs() > ERROR_INCREASE_THRESHOLD {
                        e.ai_consecutive_error_increases += 1;
                    }

                    e.ai_projectile_error += new_error;

                    if e.ai_consecutive_error_increases > ERROR_RESET_THRESHOLD {
                        e.ai_consecutive_error_increases = 0;
                        e.ai_projectile_error = 0.0;
                    }
                }
            }
        }

        entities[target_index].lifetime = 1;
        return;
    }

    for _ in 0..points {
        if update_flying_ball(target_index, entities, player) {
            break;
        }

        let entity = &mut entities[target_index];

        let x_velocity = entity.velocity.component_x * RANGED_ARC_SPEED;

        entity.coords.move_forward(entity.direction.1, x_velocity / points as f64);
    }

    let entity = &mut entities[target_index];

    // Loop for local flow control
    for _ in 0..1 {
        if !entity.on_ground {
            let mut coords = entity.coords.clone();

            coords.translate(&Coords::new(0.0, entity.velocity.component_y * RANGED_ARC_SPEED, 0.0));

            if map.test_collision(&coords, entity.entity_height) {
                map.put_on_ground(&mut entity.coords, entity.entity_height);
                entity.velocity = Velocity::zero();
                break;
            }

            entity.coords = coords;
        }
    }

    entity.velocity.accelerate(0.0, -GRAVITY);

    let rise = entity.velocity.component_y * RANGED_ARC_SPEED;
    let run = entity.velocity.component_x * RANGED_ARC_SPEED;

    entity.direction.0 = get_angle(rise, run);
}

pub fn update_flying_ball(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player) -> bool {
    let raw_entities = unsafe { &mut *(entities as *mut _) };
    let closest = get_closest_entity(target_index, &*entities);
    let hit;

    match closest {
        Some((e, d)) => {
            if d > RANGED_RADIUS {
                return false;
            }

            let damage;
            let id;
            let weapon;
            let index;
            let source_found;

            hit = true;
            debug!("Flying ball ID {} hit entity ID {}", entities[target_index].id, entities[e].id);
            // Scoped for damage call
            {
                id = unwrap_or_log!(entities[target_index].spawned_by, "Flying ball ID {} has no source", entities[target_index].id);
                let result = entities.iter().enumerate().find(|e| e.1.id == id);

                if let Some(r) = result {
                    let (i, entity) = r;

                    damage = entity.get_damage();
                    weapon = entity.current_weapon.clone();
                    index = i;
                    source_found = true;
                } else {
                    weapon = UNARMED;
                    damage = 0.0;
                    index = 0;
                    source_found = false;
                }
            }

            // If source entity not found, delete the flying ball
            if !source_found {
                entities[target_index].lifetime = 1;
                return false;
            }

            if entities[e].damage(damage, e, index, raw_entities, player) && id == player.entity_id {
                player.give_bounty(1);
            }

            if let Some(f) = weapon.on_hit {
                f(e, e, raw_entities, player);
            }
        },
        None => {
            hit = false;
        },
    }
    
    if hit {
        entities[target_index].lifetime = 1;
    }

    hit
}
