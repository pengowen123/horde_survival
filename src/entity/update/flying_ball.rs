use consts::*;
use entity::*;
use player::*;
use world::*;
use log_utils::*;
use map::*;

pub fn update_flying_ball_linear(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player) {
    let points;

    // Scoped for update_flying_ball call
    {
        let entity = &entities[target_index];
        points = entity.as_mods[0].value as usize + 1
    }
    for _ in 0..points {
        if update_flying_ball(target_index, entities, player) {
            break;
        }

        let entity = &mut entities[target_index];

        entity.coords.move_3d(entity.direction, RANGED_INTERVAL);
    }
}

pub fn update_flying_ball_arc(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player, map: &Map) {
    let points_float;
    let points;

    // Scoped for update_flying_ball call
    {
        let entity = &entities[target_index];

        if map.test_collision(&entity.coords) {
            return;
        }

        points_float = entity.velocity.component_x * 10.0;
        points = (points_float as usize + 1) / 10;
    }

    for _ in 0..points {
        if update_flying_ball(target_index, entities, player) {
            break;
        }

        let entity = &mut entities[target_index];

        let x_velocity = entity.velocity.component_x * RANGED_ARC_SPEED;

        entity.coords.move_forward(entity.direction.1, x_velocity);
    }

    let entity = &mut entities[target_index];

    // Loop for local flow control
    for _ in 0..1 {
        if !entity.on_ground {
            let mut coords = entity.coords.clone();
            coords.translate(&Coords::new(0.0, entity.velocity.component_y, 0.0));

            if map.test_collision(&coords) {
                map.put_on_ground(&mut entity.coords);
                entity.velocity = Velocity::zero();
                break;
            }

            entity.coords = coords;
        }
    }

    entity.velocity.accelerate(0.0, -GRAVITY);

    let rise = entity.velocity.component_y;
    let run = entity.velocity.component_x;

    entity.direction.0 = get_angle(rise, run);
}

pub fn update_flying_ball(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player) -> bool {
    let raw_entities = unsafe { &mut *(entities as *mut _) };
    let hit;
    let team = entities[target_index].team.clone();
    let coords = entities[target_index].coords.clone();

    match entities.iter()
                  .enumerate()
                  .filter_map(|(i, e)| {
                      if e.team != team && e.coords.in_radius(&coords, RANGED_RADIUS) {
                          Some(i)
                      } else {
                          None
                      }
                  })
                  .nth(0) {
        Some(e) => {
            let damage;
            let id;
            let weapon;
            let index;

            hit = true;
            // Scoped for damage call
            {
                id = entities[target_index].health as usize - 1;
                let (i, entity) = unwrap_or_log!(entities.iter().enumerate().find(|e| e.1.id == id),
                                                 "Source of flying ball not found");
                damage = entity.get_damage_multiplier();
                weapon = entity.current_weapon.clone();
                index = i;
            }

            if entities[e].damage(damage, e, index, raw_entities, player) && id == player.entity_id {
                player.gold += player.bounty;
            }


            if let Some(f) = weapon.on_hit {
                f(e, e, raw_entities, player);
            }
        },
        None => {
            hit = false;
        },
    }
    
    let entity = &mut entities[target_index];

    if hit {
        entity.lifetime = 1;
    }

    hit
}
