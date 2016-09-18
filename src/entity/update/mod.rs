pub mod flying_ball;
pub mod player;
pub mod ai;

use consts::*;
use player::*;
use entity::modifiers::*;
use entity::{Entity, EntityType};
use world::*;
use map::*;
use self::ai::*;

pub use self::flying_ball::*;
pub use self::player::*;

pub fn update_entity(entities: &mut Vec<Entity>, index: usize, map: &Map, player: &mut Player, next_id: &mut usize) {
    let entity_type;
    let has_ai;

    // Scoped for other entity updates
    {
        let entity = &mut entities[index];
        entity_type = entity.entity_type.clone();
        has_ai = entity.has_ai();

        entity.on_ground = map.test_collision(&entity.coords, entity.entity_height);
        update_modifiers(&mut entity.damage_mods);
        update_modifiers(&mut entity.as_mods);
        update_modifiers(&mut entity.damage_taken_mods);
        update_modifiers(&mut entity.movespeed_mods);
        update_attack_animation(&mut entity.attack_animation);
        update_lifetime(&mut entity.lifetime);
        update_gravity(&map, entity);
    }

    if has_ai {
        apply_ai(index, entities, next_id, player);
    }

    match entity_type {
        EntityType::FlyingBallLinear => {
            update_flying_ball_linear(index, entities, player);
        },
        EntityType::FlyingBallArc => {
            update_flying_ball_arc(index, entities, player, map);
        },
        _ => {},
    }
}

pub fn update_modifiers(modifiers: &mut Vec<Modifier>) {
    *modifiers = modifiers.iter().cloned().filter(|m| !m.is_expired()).collect();

    for modifier in modifiers {
        modifier.update();
    }
}

pub fn update_attack_animation(timer: &mut usize) {
    if *timer > 0 {
        *timer -= 1;
    }
}

pub fn update_lifetime(timer: &mut usize) {
    if *timer > 1 {
        *timer -= 1;
    }
}

pub fn update_gravity(map: &Map, entity: &mut Entity) {
    if EntityType::FlyingBallArc == entity.entity_type && map.test_collision(&entity.coords, entity.entity_height) {
        entity.lifetime = 1;
        return;
    }

    if entity.has_gravity() {
        let mut coords = entity.coords.clone();
        coords.translate(&Coords::new(0.0, entity.velocity.component_y, 0.0));

        if map.test_collision(&coords, entity.entity_height) {
            map.put_on_ground(&mut entity.coords, entity.entity_height);
            entity.velocity.component_y = 0.0;
            return;
        }

        entity.coords = coords;
        entity.velocity.accelerate(0.0, -GRAVITY);
    }
}
