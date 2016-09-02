use player::Player;
use world::*;
use entity::*;
use items::*;
use consts::balance::*;
use log_utils::*;

pub fn attack_melee_area(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player) -> usize {
    let raw_entities = unsafe { &mut *(entities as *mut _) };
    let mut killed = 0;
    let point;
    let range;
    let team;
    let multiplier;
    let weapon;

    // Scoped for iter_mut call
    {
        let entity = &entities[target_index];
        range = entity.current_weapon.range;
        team = entity.team.clone();
        point = unwrap_or_log!(entity.coords.ray(entity.current_weapon.range, entity.direction.clone()).nth(1),
                               "Ray was deleted");
        multiplier = entity.get_damage_multiplier();
        weapon = entity.current_weapon.clone();
    }

    let affected = entities.iter_mut().enumerate().filter(|&(_, ref e)| {
        e.coords.in_radius(&point, range) && e.team != team && !e.is_dummy()
    });

    for (i, e) in affected {
        e.damage(multiplier, i, target_index, raw_entities, player);

        if let Some(f) = weapon.on_hit {
            f(i, target_index, raw_entities, player);
        }

        killed += e.is_dead() as i32 as usize;
    }

    killed
}

pub fn attack_melee_line(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player) -> usize {
    let mut points;
    let mut killed = 0;
    let raw_entities = unsafe { &mut *(entities as *mut _) };
    let multiplier;
    let coords;
    let direction;
    let team;
    let weapon;

    // Scoped for iter_mut call
    {
        let entity = &entities[target_index];

        // If this is 0, the weapon cannot hit anything
        points = entity.current_weapon.range as usize;
        multiplier = entity.get_damage_multiplier();
        coords = entity.coords.clone();
        direction = entity.direction.clone();
        team = entity.team.clone();
        weapon = entity.current_weapon.clone();
    }

    for point in coords.ray(MELEE_LINE_INTERVAL, direction).skip(1) {
        match entities.iter_mut().enumerate().filter(|&(_, ref e)| {
            e.coords.in_radius(&point, MELEE_LINE_RADIUS) && e.team != team && !e.is_dummy()
        }).nth(0) {
            Some((i, e)) => {
                e.damage(multiplier, i, target_index, raw_entities, player);

                if let Some(f) = weapon.on_hit {
                    f(i, target_index, raw_entities, player);
                }

                killed = e.is_dead() as i32 as usize;
                break;
            },
            None => {},
        }

        if points > 0 {
            points -= 1;
        }

        if points == 0 {
            break;
        }
    }

    killed
}

pub fn attack_ranged_linear(target_index: usize, entities: &mut Vec<Entity>, next_id: &mut usize) -> usize {
    let mut dummy;

    // Scoped for push call
    {
        let entity = &entities[target_index];
        dummy = Entity::new(*next_id,
                            (entity.id + 1) as f64,
                            0.0,
                            entity.coords.clone(),
                            EntityType::FlyingBallLinear,
                            entity.team.clone(),
                            IsDummy::True,
                            entity.direction.clone(),
                            RANGED_LINEAR_LIFETIME,
                            HasGravity::False,
                            HasAI::False);

        *next_id += 1;

        dummy.as_mods.push(Modifier::new(entity.current_weapon.range, 0));
    }

    entities.push(dummy);

    0
}

pub fn attack_ranged_projectile(target_index: usize, entities: &mut Vec<Entity>, next_id: &mut usize) -> usize {
    let mut dummy;

    // Scoped for push call
    {
        let entity = &mut entities[target_index];
        dummy = Entity::new(*next_id,
                            (entity.id + 1) as f64,
                            0.0,
                            entity.coords.clone(),
                            EntityType::FlyingBallArc,
                            entity.team.clone(),
                            IsDummy::True,
                            entity.direction.clone(),
                            RANGED_ARC_LIFETIME,
                            HasGravity::False,
                            HasAI::False);

        *next_id += 1;

        let speed = entity.current_weapon.range;
        let angle = Direction((entity.direction.0 - 90.0).abs()).as_radians();
        dummy.velocity.accelerate(angle.cos() * speed, angle.sin() * speed);
        dummy.coords.y += PROJECTILE_SPAWN_OFFSET;
    }

    entities.push(dummy);

    0
}

pub fn try_attack(id: usize, entities: &mut Vec<Entity>, next_id: &mut usize, player: &mut Player) -> usize {
    // NOTE: Dummy entities created by ranged attacks use their health to store the id of the
    //       entity they were created by
    //
    // NOTE: health = id + 1

    let index = entities.iter().find(|e| e.id == id).expect(&format!("Entity not found: {}", id)).id;
    let weapon_type;
    
    // Scoped for attack function calls
    {
        let entity = &mut entities[index];
        weapon_type = entity.current_weapon.weapon_type.clone();

        if entity.attack_animation > 0 {
            return 0;
        }

        let attack_speed = entity.as_mods.iter().fold(entity.current_weapon.attack_speed, |acc, x| acc * x.value);
        entity.attack_animation = entity.current_weapon.get_attack_time(attack_speed);
    }

    match weapon_type {
        WeaponType::MeleeArea => attack_melee_area(index, entities, player),
        WeaponType::MeleeLine => attack_melee_line(index, entities, player),
        WeaponType::RangedLinear => attack_ranged_linear(index, entities, next_id),
        WeaponType::RangedProjectile => attack_ranged_projectile(index, entities, next_id),
    }
}
