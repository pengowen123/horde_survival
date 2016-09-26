use player::Player;
use world::*;
use entity::*;
use items::*;
use consts::balance::*;
use hslog::*;

pub fn attack_melee_area(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player) -> usize {
    let raw_entities = unsafe { &mut *(entities as *mut _) };
    let mut bounty = 0;
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
        multiplier = entity.get_damage();
        weapon = entity.current_weapon.clone();
    }

    let affected = entities.iter_mut().enumerate().filter(|&(_, ref e)| {
        e.coords.in_radius(&point, range) && e.team != team && !e.is_dummy()
    });

    for (i, e) in affected {
        e.damage(multiplier, i, target_index, raw_entities, player);

        if let Some(f) = weapon.on_hit {
            f(target_index, i, raw_entities, player);
        }

        if e.is_dead() {
            bounty += e.bounty;
        }
    }

    bounty
}

pub fn attack_melee_line(target_index: usize, entities: &mut Vec<Entity>, player: &mut Player) -> usize {
    let mut points;
    let mut bounty = 0;
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
        multiplier = entity.get_damage();
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
                    f(target_index, i, raw_entities, player);
                }

                if e.is_dead() {
                    bounty = e.bounty;
                }

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

    bounty
}

pub fn attack_ranged_linear(target_index: usize, entities: &mut Vec<Entity>, next_id: &mut usize) -> usize {
    let mut dummy;

    // Scoped for push call
    {
        let entity = &entities[target_index];
        dummy = Entity::new(*next_id,
                            1.0,
                            1.0,
                            entity.coords.translated(0.0, PROJECTILE_SPAWN_OFFSET, 0.0),
                            EntityType::FlyingBallLinear,
                            entity.team.clone(),
                            IsDummy::True,
                            entity.direction.clone(),
                            RANGED_LINEAR_LIFETIME,
                            0,
                            HasGravity::False,
                            HasAI::False,
                            Some(entity.id));

        *next_id += 1;

        dummy.as_mods.push(Modifier::additive(entity.current_weapon.range, 0));
    }

    entities.push(dummy);

    0
}

pub fn attack_ranged_projectile(target_index: usize, entities: &mut Vec<Entity>, next_id: &mut usize) -> usize {
    let mut dummy;

    // Scoped for push call
    {
        let entity = &entities[target_index];
        dummy = Entity::new(*next_id,
                            1.0,
                            1.0,
                            entity.coords.translated(0.0, PROJECTILE_SPAWN_OFFSET, 0.0),
                            EntityType::FlyingBallArc,
                            entity.team.clone(),
                            IsDummy::True,
                            entity.direction.clone(),
                            INFINITE_LIFETIME,
                            0,
                            HasGravity::False,
                            HasAI::False,
                            Some(entity.id));

        *next_id += 1;

        let speed = entity.current_weapon.range * RANGED_ARC_SPEED;
        let angle = Direction(entity.direction.0 - 90.0).as_radians();
        dummy.velocity.accelerate(angle.cos() * speed, angle.sin() * speed);
    }

    entities.push(dummy);

    0
}

// NOTE: Do not call this function directly, instead set the entities `attack` field to true
pub fn try_attack(id: usize, entities: &mut Vec<Entity>, next_id: &mut usize, player: &mut Player) -> usize {
    let index = unwrap_or_log!(entities.iter().enumerate().find(|&(_, e)| e.id == id),
                               "Entity not found: {}", id).0;
    let weapon_type;
    let is_casting;
    
    // Scoped for attack function calls
    {
        let entity = &mut entities[index];
        weapon_type = entity.current_weapon.weapon_type.clone();
        is_casting = entity.animations.is_casting(0);

        if !(entity.animations.can_attack() && !is_casting) {
            if !is_casting {
                return 0;
            }
        }

        let attack_speed = apply(&entity.as_mods, entity.current_weapon.attack_speed);
        let attack_time = entity.current_weapon.get_attack_time(attack_speed);
        let pre = entity.current_weapon.anim_pre as f64 * attack_time;
        let post = entity.current_weapon.anim_post as f64 * attack_time;

        entity.animations.start(0, pre as usize, post as usize);
    }

    if is_casting {
        match weapon_type {
            WeaponType::MeleeArea => attack_melee_area(index, entities, player),
            WeaponType::MeleeLine => attack_melee_line(index, entities, player),
            WeaponType::RangedLinear => attack_ranged_linear(index, entities, next_id),
            WeaponType::RangedProjectile => attack_ranged_projectile(index, entities, next_id),
        }
    } else {
        0
    }
}
