use consts::*;
use entity::*;
use player::*;
use world::*;

pub fn update_flying_ball_linear(target_index: usize, entities: &mut [Entity], player: &mut Player) {
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

pub fn update_flying_ball_arc(target_index: usize, entities: &mut [Entity], player: &mut Player) {
    let points_float;
    let points;

    // Scoped for update_flying_ball call
    {
        let entity = &entities[target_index];
        points_float = entity.velocity.component_x * 10.0;
        points = (points_float as usize + 1) / 10;
    }

    for _ in 0..points {
        // TODO: Set direction of the entity based on rise and run
        if update_flying_ball(target_index, entities, player) {
            break;
        }

        let entity = &mut entities[target_index];

        let x_velocity = entity.velocity.component_x * RANGED_ARC_SPEED;

        entity.coords.move_forward(entity.direction.1, x_velocity);
    }

    let entity = &mut entities[target_index];

    if !entity.on_ground {
        entity.coords.translate(&Coords::new(0.0, entity.velocity.component_y, 0.0));
    }

    entity.velocity.accelerate(0.0, -GRAVITY);

    let rise = entity.velocity.component_y.abs();
    let negative_rise = rise < 0.0;
    let run = entity.velocity.component_x;

    let angle = 0.0;

    if negative_rise {
        entity.direction.0 = 90.0 + angle;
    } else {
        entity.direction.0 = 90.0 - angle;
    }
}

pub fn update_flying_ball(target_index: usize, entities: &mut [Entity], player: &mut Player) -> bool {
    let hit;
    let is_enemy = entities[target_index].is_enemy;
    let coords = entities[target_index].coords.clone();

    match entities.iter()
                  .enumerate()
                  .filter_map(|(i, e)| {
                      if e.is_enemy != is_enemy && e.coords.in_radius(&coords, RANGED_RADIUS) {
                          Some(i)
                      } else {
                          None
                      }
                  })
                  .nth(0) {
        Some(e) => {
            let damage;
            let id;
            hit = true;
            // Scoped for attack_entity call
            {
                let entity = &entities[target_index];
                let weapon_damage = entity.current_weapon.damage;
                damage = entity.damage_mods.iter().fold(weapon_damage, |acc, x| acc * x.value);
                id = entity.health as usize - 1;
            }

            if entities[e].damage(damage) && id == player.entity_id {
                player.gold += player.bounty;
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
