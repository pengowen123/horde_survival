//! Update functions for entities, called every tick

pub mod flying_ball;
pub mod player;
pub mod ai;

pub use self::flying_ball::*;
pub use self::player::*;

use consts::*;
use player::*;
use entity::modifiers::*;
use entity::{Entity, EntityType};
use world::*;
use map::*;
use self::ai::*;
use entity::try_attack;

/// Generic update function that also calls entity specific update functions
pub fn update_entity(entities: &mut Vec<Entity>,
                     index: usize,
                     map: &Map,
                     player: &mut Player,
                     next_id: &mut usize) {

    // Run the AI if the entity uses it
    if entities[index].has_ai() {
        apply_ai(index, entities);
    }

    let entity_type;
    let is_attacking;
    let id;
    let coords;
    let attack;

    // Scoped for other entity updates
    {
        let entity = &mut entities[index];
        entity_type = entity.entity_type;
        id = entity.id;
        coords = entity.coords;

        // Update hitbox
        entity.update_hitbox();
        // Update on_ground flag
        entity.on_ground = map.test_collision(&entity.coords, entity.get_height());
        // Update modifiers
        update_modifiers(&mut entity.damage_mods);
        update_modifiers(&mut entity.as_mods);
        update_modifiers(&mut entity.damage_taken_mods);
        update_modifiers(&mut entity.movespeed_mods);
        // Update lifetime counter
        update_lifetime(&mut entity.lifetime);
        // Apply gravity to the entity (the check for whether the entity has gravity is run in the
        // function)
        update_gravity(map, entity);

        // If the entity isn't a dummy, update its animations and test whether it is attacking
        if !entity.is_dummy() {
            entity.animations.update();
            is_attacking = entity.animations.is_attacking();
        } else {
            is_attacking = false;
        }

        attack = entity.attack;
        entity.attack = false;
    }

    // If the attack animation is playing, or if the entity is trying to attack, try to attack
    // try_attack does some additional tests, so the attack won't happen unless it is valid
    if is_attacking || attack {
        let gold_gained = try_attack(id, entities, next_id, player);

        if id == player.entity_id {
            player.give_gold(gold_gained);
        }
    }

    // Call entity specific update functions
    match entity_type {
        EntityType::FlyingBallLinear => {
            update_flying_ball_linear(index, entities, player);
        }
        EntityType::FlyingBallArc => {
            update_flying_ball_arc(index, entities, player, map);
        }
        _ => {}
    }

    // Sets whether the entity has moved enough to require a graphical update
    let entity = &mut entities[index];
    entity.needs_update = coords.distance(&entity.coords) > UPDATE_THRESHOLD;
}

/// Update a list of modifiers
pub fn update_modifiers(modifiers: &mut Vec<Modifier>) {
    *modifiers = modifiers.iter().cloned().filter(|m| !m.is_expired()).collect();

    for modifier in modifiers {
        modifier.update();
    }
}

/// Update an entity's lifetime counter
pub fn update_lifetime(timer: &mut usize) {
    if *timer > 1 {
        *timer -= 1;
    }
}

/// Apply gravity to an entity
pub fn update_gravity(map: &Map, entity: &mut Entity) {
    // Only apply gravity if the entity has it enabled
    if entity.has_gravity() {
        // Change the entity's vertical position by its vertical velocity
        let mut coords = entity.coords;
        coords.translate(&Coords::new(0.0, entity.velocity.component_y, 0.0));
        let height = entity.get_height();

        // If the entity's new coordinates are in the ground, instead place the entity on the ground
        if map.test_collision(&coords, height) {
            map.put_on_ground(&mut entity.coords, height);
            entity.velocity.component_y = 0.0;
            return;
        }

        // Apply the changes to the entity's position, and apply acceleration due to gravity
        entity.coords = coords;
        entity.velocity.accelerate(0.0, -GRAVITY);
    }
}

/// Moves clumped entities away from each other
// FIXME: This function is unused because it causes entities to violently shake and move forever in
//        one direction against their will
#[allow(dead_code)]
pub fn update_clumped_entities(entities: &mut [Entity]) {
    // Move these two lines to their appropriate places when the function is fixed
    use rand::Rng;
    const UNCLUMP_RADIUS: f64 = 0.2;

    let distance = UNCLUMP_RADIUS / 5.0;
    let mut angles = Vec::new();
    let mut rng = ::rand::thread_rng();

    for entity in entities.iter() {
        if entity.is_dummy() {
            angles.push(None);
            continue;
        }


        let mut angle = None;

        for other in entities.iter().filter(|e| !e.is_dummy()) {
            if other.is_dummy() {
                continue;
            }


            let new_angle = if entity.coords.distance(&other.coords) <= UNCLUMP_RADIUS {
                let dx = entity.coords.x - other.coords.x;
                let dy = entity.coords.y - other.coords.y;
                get_angle2(dx, dy)
            } else {
                0.0
            };



            if let Some(ref mut a) = angle {
                *a += new_angle;
            } else {
                angle = Some(new_angle);
            }
        }



        if let Some(ref mut a) = angle {
            *a /= entities.len() as f64;
            *a += rng.gen::<f64>() * 10.0;

            if rng.gen() {
                *a += 180.0;


                angles.push(Some(*a));
            } else {
                angles.push(None);
            }
        }
    }


    for (i, entity) in entities.iter_mut().enumerate() {
        if let Some(angle) = angles[i] {
            entity.coords.move_forward(Direction(angle).wrap().0, distance);
        }
    }
}
