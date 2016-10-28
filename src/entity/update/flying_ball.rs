use consts::*;
use entity::*;
use entity::update::ai::calculate_error;
use player::*;
use world::*;
use map::*;
use hslog::CanUnwrap;

pub fn update_flying_ball_linear(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player) {
    let speed;

    // Scoped for update_flying_ball call
    {
        let entity = &mut entities[target_index];
        speed = entity.as_mods[0].value;

        if entity.on_ground {
            entity.lifetime = 1;
            return;
        }
    }

    let old;
    let new;
    // Scoped for update_flying_ball call
    {
        let entity = &mut entities[target_index];

        // NOTE: Uncomment this
        //if entity.on_ground {
            //entity.lifetime = 1;
            //return;
        //}

        old = entity.coords.clone();
        entity.coords.move_3d(entity.direction, speed * RANGED_INTERVAL * RANGED_LINEAR_SPEED);

        new = entity.coords.clone();
    }

    update_flying_ball(target_index, entities, old, new, player);
}

pub fn update_flying_ball_arc(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player, map: &Map) {
    if entities[target_index].on_ground {
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
                    new_error *= (0.5 / PROJECTILE_LEARNING_RATE).powi(4);

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

    let old;
    let new;
    // Scoped for update_flying_ball call
    {
        let entity = &mut entities[target_index];
        old = entity.coords.clone();
        let x_velocity = entity.velocity.component_x * RANGED_ARC_SPEED;
        entity.coords.move_forward(entity.direction.1, x_velocity as f64);

        // Loop for local flow control
        loop {
            if !entity.on_ground {
                let mut coords = entity.coords.clone();

                coords.translate(&Coords::new(0.0, entity.velocity.component_y * RANGED_ARC_SPEED, 0.0));
                let height = entity.get_height();

                if map.test_collision(&coords, height) {
                    map.put_on_ground(&mut entity.coords, height);
                    entity.velocity = Velocity::zero();
                    break;
                }

                entity.coords = coords;
            }

            break;
        }

        entity.velocity.accelerate(0.0, -GRAVITY);

        let rise = entity.velocity.component_y;
        let run = entity.velocity.component_x;

        entity.direction.0 = get_angle(rise, run);

        new = entity.coords.clone();
    }

    update_flying_ball(target_index, entities, old, new, player);
}

pub fn update_flying_ball(target_index: usize,
                          entities: &mut Vec<Entity>,
                          old_pos: Coords,
                          new_pos: Coords,
                          player: &mut Player) -> bool {

    // NOTE: Hopefully this doesn't cause problems
    let raw_entities = unsafe { &mut *(entities as *mut Vec<Entity>) };
    let collided = get_collided_entity(target_index, entities, old_pos, new_pos);
    let hit;

    match collided {
        Some(e) => {
            let damage;
            let id;
            let weapon;
            let index;

            hit = true;
            // Scoped for damage call
            {
                // NOTE: If this fails, it means that the spawned_by field was None. This is
                //       different from the source entity not being found
                id = unwrap_or_log!(entities[target_index].spawned_by, "Flying ball ID {} has no source", entities[target_index].id);
                let result = entities.iter().enumerate().find(|e| e.1.id == id);

                if let Some(r) = result {
                    let (i, entity) = r;

                    damage = entity.get_damage();
                    weapon = entity.current_weapon.clone();
                    index = i;
                } else {
                    // If source entity not found, delete the flying ball
                    raw_entities[target_index].lifetime = 1;
                    return false;
                }
            }

            let entity = &mut entities[e];

            if entity.damage(damage, e, index, raw_entities, player) && id == player.entity_id {
                player.give_gold(entity.bounty);
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
