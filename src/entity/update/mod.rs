pub mod flying_ball;
pub mod player;
pub mod ai;

pub use self::flying_ball::*;
pub use self::player::*;

//use rand::{thread_rng, Rng};

use consts::*;
use player::*;
use entity::modifiers::*;
use entity::{Entity, EntityType};
use world::*;
use map::*;
use self::ai::*;
use entity::try_attack;

pub fn update_entity(entities: &mut Vec<Entity>, index: usize, map: &Map, player: &mut Player, next_id: &mut usize) {
    let entity_type;
    let is_casting;
    let id;
    let coords;
    let attack;

    if entities[index].has_ai() {
        apply_ai(index, entities);
    }

    // Scoped for other entity updates
    {
        let entity = &mut entities[index];
        entity_type = entity.entity_type.clone();
        id = entity.id;
        coords = entity.coords.clone();

        entity.update_hitbox();
        entity.on_ground = map.test_collision(&entity.coords, entity.get_height());
        update_modifiers(&mut entity.damage_mods);
        update_modifiers(&mut entity.as_mods);
        update_modifiers(&mut entity.damage_taken_mods);
        update_modifiers(&mut entity.movespeed_mods);
        update_lifetime(&mut entity.lifetime);
        update_gravity(&map, entity);

        if !entity.is_dummy() {
            entity.animations.update();
            is_casting = entity.animations.is_casting(0);
        } else {
            is_casting = false;
        }

        attack = entity.attack;
        entity.attack = false;
    }

    if is_casting || attack {
        let gold_gained = try_attack(id, entities, next_id, player);

        if id == player.entity_id {
            player.give_gold(gold_gained);
        }
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

    let entity = &mut entities[index];

    entity.needs_update = coords.distance(&entity.coords) > UPDATE_THRESHOLD;
}

pub fn update_modifiers(modifiers: &mut Vec<Modifier>) {
    *modifiers = modifiers.iter().cloned().filter(|m| !m.is_expired()).collect();

    for modifier in modifiers {
        modifier.update();
    }
}

pub fn update_lifetime(timer: &mut usize) {
    if *timer > 1 {
        *timer -= 1;
    }
}

pub fn update_gravity(map: &Map, entity: &mut Entity) {
    if entity.has_gravity() {
        let mut coords = entity.coords.clone();
        coords.translate(&Coords::new(0.0, entity.velocity.component_y, 0.0));
        let height = entity.get_height();

        if map.test_collision(&coords, height) {
            map.put_on_ground(&mut entity.coords, height);
            entity.velocity.component_y = 0.0;
            return;
        }

        entity.coords = coords;
        entity.velocity.accelerate(0.0, -GRAVITY);
    }
}

// TODO: Fix this
//pub fn update_clumped_entities(entities: &mut [Entity]) {
    //let distance = UNCLUMP_RADIUS / 5.0;
    //let mut angles = Vec::new();
    //let mut rng = thread_rng();

    //for entity in entities.iter() {
        //if entity.is_dummy() {
            //angles.push(None);
            //continue;
        //}

        //let mut angle = None;

        //for other in entities.iter().filter(|e| !e.is_dummy()) {
            //if other.is_dummy() {
                //continue;
            //}

            //let new_angle;

            //if entity.coords.distance(&other.coords) <= UNCLUMP_RADIUS {
                //let dx = entity.coords.x - other.coords.x;
                //let dy = entity.coords.y - other.coords.y;
                //new_angle = get_angle2(dx, dy);
            //} else {
                //new_angle = 0.0;
            //}

            //if let Some(ref mut a) = angle {
                //*a += new_angle;
            //} else {
                //angle = Some(new_angle);
            //}
        //}

        //if let Some(ref mut a) = angle {
            //*a /= entities.len() as f64;
            //*a += rng.gen::<f64>() * 10.0;

            //if rng.gen() {
                //*a += 180.0;
            //}

            //angles.push(Some(*a));
        //} else {
            //angles.push(None);
        //}
    //}

    //for (i, entity) in entities.iter_mut().enumerate() {
        //if let Some(angle) = angles[i] {
            //entity.coords.move_forward(Direction(angle).wrap().0, distance);
        //}
    //}
//}
