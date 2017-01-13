//! Update functions for projectiles from weapons

use consts::*;
use entity::*;
use entity::update::ai::calculate_error;
use player::*;
use world::*;
use map::*;

/// Update function for `FlyingBallLinear` entities
pub fn update_flying_ball_linear(target_index: usize,
                                 entities: &mut Vec<Entity>,
                                 player: &mut Player) {
    let speed;
    let old;
    let new;

    // Scoped for update_flying_ball call
    {
        let entity = &mut entities[target_index];
        speed = entity.as_mods[0].value;

        // Delete the projectile if it hit the map
        // TODO: This may be affected by the implementation of terrain collision detection
        if entity.on_ground {
            entity.lifetime = 1;
            return;
        }

        old = entity.coords;
        // Update the projectile's position
        entity.coords.move_3d(entity.direction,
                              speed * RANGED_INTERVAL * RANGED_LINEAR_SPEED);

        new = entity.coords;
    }

    // Additional updates
    update_flying_ball(target_index, entities, old, new, player);
}

/// Update function for `FlyingBallArc` entities
pub fn update_flying_ball_arc(target_index: usize,
                              entities: &mut Vec<Entity>,
                              player: &mut Player,
                              map: &Map) {

    // If the projectile has landed, update information used by the AI
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
            projectile_coords = entity.coords;
            id = entity.id;

            // Get the intended target entity of the AI
            let target = entities.iter().find(|e| e.id == entity.ai_target_id);

            if let Some(e) = target {
                target_coords = e.coords;
            } else {
                found_target = false;
                // This value isn't used (target_coords isn't read if no target was found)
                target_coords = Default::default();
            }
        }

        if found_target {
            // Get the source of the projectile
            let source_id =
                spawned_by.unwrap_or_else(|| crash!("Flying ball ID {} has no source", id));
            let source = entities.iter_mut().find(|e| e.id == source_id);

            // If the source exists, and is AI controlled
            if let Some(e) = source {
                if e.has_ai() {
                    // Get the new error
                    let mut new_error =
                        PROJECTILE_LEARNING_RATE *
                        calculate_error(&e.coords, &projectile_coords, &target_coords);
                    // Apply some adjustments (these were found by trial and error)
                    new_error *= (0.5 / PROJECTILE_LEARNING_RATE).powi(4);

                    // Get how much the error changed
                    let increase = if e.ai_projectile_error > 0.0 {
                        new_error - e.ai_projectile_error
                    } else {
                        e.ai_projectile_error - new_error
                    };

                    // If the error increased too much, update the failed error update counter
                    if increase.abs() > ERROR_INCREASE_THRESHOLD {
                        e.ai_consecutive_error_increases += 1;
                    }

                    // Apply the new error
                    e.ai_projectile_error += new_error;

                    // If there were too many failed error updates, reset the error and start over
                    // This is to prevent the AI from spiraling out of control from inaccurate error
                    // updates, causing it to only shoot straight up or down
                    if e.ai_consecutive_error_increases > ERROR_RESET_THRESHOLD {
                        e.ai_consecutive_error_increases = 0;
                        e.ai_projectile_error = 0.0;
                    }
                }
            }
        }

        // Delete the projectile if it hit the map
        entities[target_index].lifetime = 1;
        return;
    }

    let old;
    let new;
    // Scoped for update_flying_ball call
    {
        let entity = &mut entities[target_index];
        old = entity.coords;
        // Update the projectile's horizontal position
        let x_velocity = entity.velocity.component_x * RANGED_ARC_SPEED;
        entity.coords.move_forward(entity.direction.1, x_velocity as f64);

        // Apply gravity to the entity, and update its vertical position
        // Gravity is disabled for FlyingBallArc entities to allow a custom implementation
        // (specifically, the entity's y velocity is multiplied by RANGED_ARC_SPEED)

        // Loop for local flow control
        loop {
            if !entity.on_ground {
                let mut coords = entity.coords;

                coords.translate(&Coords::new(0.0,
                                              entity.velocity.component_y * RANGED_ARC_SPEED,
                                              0.0));

                let height = entity.get_height();

                if map.test_collision(&coords, height) {
                    map.put_on_ground(&mut entity.coords, height);
                    entity.velocity = Default::default();
                    break;
                }

                entity.coords = coords;
            }

            break;
        }

        entity.velocity.accelerate(0.0, -GRAVITY);

        // Calculate the direction the projectile is facing based on the direction it traveled in
        let rise = entity.velocity.component_y;
        let run = entity.velocity.component_x;
        entity.direction.0 = get_angle(rise, run);

        new = entity.coords;
    }

    // Additional updates
    update_flying_ball(target_index, entities, old, new, player);
}

/// Updates to be applied to both `FlyingBallLinear` and `FlyingBallProjectile` entities
pub fn update_flying_ball(target_index: usize,
                          entities: &mut Vec<Entity>,
                          old_pos: Coords,
                          new_pos: Coords,
                          player: &mut Player)
                          -> bool {

    // NOTE: Hopefully this doesn't cause problems
    //       There is a double borrow documented below
    // FIXME: This can be removed by cloning the entity, see fn warrior_ability_3
    let raw_entities = unsafe { &mut *(entities as *mut Vec<Entity>) };

    // The index of the entity the projectile hit, if any
    let collided = get_collided_entity(target_index, entities, old_pos, new_pos);
    let hit;

    match collided {
        // The projectile has hit an entity
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
                //       If the source entity was not found, the field would be Some, but that
                //       entity has died since spawning this projectile
                //       If it is None, that means this projectile was spawned incorrectly
                let entity = &entities[target_index];
                id = entity.spawned_by
                    .unwrap_or_else(|| crash!("Flying ball ID {} has no source", entity.id));
                let source = entities.iter().enumerate().find(|e| e.1.id == id);

                if let Some(r) = source {
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

            // NOTE: The double borrow occurs here
            //       This is safe as long as the collided entity is not accessed by anything in
            //       the Entity::damage call
            //       That could happen if the entity hit by the projectile is the same as the source
            //       of the projectile, which should never happen due to checks in
            //       get_collided_entity
            let entity = &mut entities[e];
            if entity.damage(damage, e, index, raw_entities, player) && id == player.entity_id {
                player.give_gold(entity.bounty);
            }

            // Apply on hit modifiers from the source entity's weapon
            if let Some(f) = weapon.on_hit {
                f(e, index, raw_entities, player);
            }
        }
        // No entity was hit by the projectile
        None => {
            hit = false;
        }
    }

    // Delete the projectile if it hit an entity
    if hit {
        entities[target_index].lifetime = 1;
    }

    hit
}
