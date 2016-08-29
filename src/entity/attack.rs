use entity::*;
use items::*;
use consts::balance::*;

pub fn attack_melee_area(entity: &mut Entity, entities: &mut Vec<Entity>) -> usize {
    let mut killed = 0;
    let point = entity.coords.ray(entity.current_weapon.range, entity.direction.clone()).nth(1).expect("Ray was deleted");

    let affected = entities.iter_mut().filter(|e| {
        e.coords.in_radius(&point, entity.current_weapon.range) && e.is_enemy != entity.is_enemy && !e.dummy
    });

    for e in affected {
        entity.attack_entity(e);
        killed += e.is_dead() as i32 as usize;
    }

    killed
}

pub fn attack_melee_line(entity: &mut Entity, entities: &mut Vec<Entity>) -> usize {
    let mut killed = 0;
    let mut points = entity.current_weapon.range as usize;

    for point in entity.coords.ray(ATTACK_MELEE_LINE_INTERVAL, entity.direction.clone()).skip(1) {
        match entities.iter_mut().filter(|e| {
            e.coords.in_radius(&point, ATTACK_MELEE_LINE_RADIUS) && e.is_enemy != entity.is_enemy && !e.dummy
        }).nth(0) {
            Some(e) => {
                entity.attack_entity(e);
                killed = e.is_dead() as i32 as usize;
                break;
            },
            None => {},
        }

        points -= 1;

        if points == 0 {
            break;
        }
    }

    killed
}

pub fn attack_ranged_linear(entity: &mut Entity, entities: &mut Vec<Entity>, next_id: &mut usize) -> usize {
    let mut dummy = Entity::new(*next_id,
                            (entity.id + 1) as f64,
                            entity.coords.clone(),
                            EntityType::FlyingBallLinear,
                            false,
                            true,
                            entity.direction.clone(),
                            RANGED_LINEAR_LIFETIME * LIFETIME_MULTIPLIER);

    *next_id += 1;
    dummy.as_mods.push(Modifier::new(entity.current_weapon.range, 0));
    entities.push(dummy);

    0
}

pub fn attack_ranged_projectile(entity: &mut Entity, entities: &mut Vec<Entity>, next_id: &mut usize) -> usize {
    let mut dummy = Entity::new(*next_id,
                            (entity.id + 1) as f64,
                            entity.coords.clone(),
                            EntityType::FlyingBallArc,
                            false,
                            true,
                            entity.direction.clone(),
                            0);

    *next_id += 1;
    dummy.as_mods.push(Modifier::new(entity.current_weapon.range, 0));
    entities.push(dummy);

    0
}

pub fn try_attack(id: usize, entities: &mut Vec<Entity>, next_id: &mut usize) -> usize {
    // NOTE: Dummy entities created by ranged attacks use their health to store the id of the
    //       entity they were created by
    //       Projectile speed is stored in entity.as_mods[0].value
    //
    // NOTE: health = id + 1

    let entity = unsafe {
        // I'm sorry for this
        // If it causes any issues, it might be necessary to clone the entity before attacking
        &mut *(entities.iter_mut().find(|e| e.id == id).expect(&format!("Entity not found: {}", id)) as *mut Entity)
    };
    
    // FIXME: attack animation is really broken
    // NOTE: or is it?
    // NOTE: yes it is
    if entity.attack_animation > 0 {
        return 0;
    }

    println!("attacking");

    let attack_speed = entity.as_mods.iter().fold(entity.current_weapon.attack_speed, |acc, x| acc * x.value);
    entity.attack_animation = get_attack_time(attack_speed);

    match entity.current_weapon.weapon_type {
        WeaponType::MeleeArea => attack_melee_area(entity, entities),
        WeaponType::MeleeLine => attack_melee_line(entity, entities),
        WeaponType::RangedLinear => attack_ranged_linear(entity, entities, next_id),
        WeaponType::RangedProjectile => attack_ranged_projectile(entity, entities, next_id),
    }
}
