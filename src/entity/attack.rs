use player::Player;
use world::*;
use entity::*;
use items::*;
use consts::balance::*;

/// Represents an attack by a MeleeArea weapon
pub fn attack_melee_area(target_index: usize,
                         entities: &mut Vec<Entity>,
                         player: &mut Player)
                         -> usize {
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
        point = entity.coords
            .ray(entity.current_weapon.range, entity.direction.clone())
            .nth(1)
            .unwrap_or_else(|| crash!("Ray was deleted"));
        multiplier = entity.get_damage();
        weapon = entity.current_weapon.clone();

        assert!(weapon.weapon_type == WeaponType::MeleeArea,
                "Can only call attack_melee_area with a
                weapon of type MeleeArea");
    }

    let affected = entities.iter_mut()
        .enumerate()
        .filter(|&(_, ref e)| e.coords.in_radius(&point, range) && e.team != team && !e.is_dummy());

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

/// Represents an attack by a MeleeLine weapon
pub fn attack_melee_line(target_index: usize,
                         entities: &mut Vec<Entity>,
                         player: &mut Player)
                         -> usize {
    let mut bounty = 0;
    let raw_entities = unsafe { &mut *(entities as *mut _) };
    let multiplier;
    let coords;
    let direction;
    let weapon;

    // Scoped for iter_mut call
    {
        let entity = &entities[target_index];

        // If this is 0, the weapon cannot hit anything
        multiplier = entity.get_damage();
        coords = entity.coords.clone();
        direction = entity.direction;
        weapon = entity.current_weapon.clone();

        assert!(weapon.weapon_type == WeaponType::MeleeLine,
                "Can only call attack_melee_line with a
                weapon of type MeleeLine");
    }

    let mut new = coords.clone();
    new.move_3d(direction, weapon.range);

    if let Some(e) = get_collided_entity(target_index, &*entities, coords, new) {
        let entity = &mut entities[e];

        entity.damage(multiplier, e, target_index, raw_entities, player);

        if let Some(f) = weapon.on_hit {
            f(target_index, e, raw_entities, player);
        }

        if entity.is_dead() {
            bounty = entity.bounty;
        }
    }

    bounty
}

/// Represents an attack with a RangedLinear weapon
pub fn attack_ranged_linear(target_index: usize,
                            entities: &mut Vec<Entity>,
                            next_id: &mut usize)
                            -> usize {
    let mut dummy;

    // Scoped for push call
    {
        let entity = &entities[target_index];

        assert!(entity.current_weapon.weapon_type == WeaponType::RangedLinear,
                "Can only call attack_ranged_linear with a
                weapon of type RangedLinear");

        dummy = Entity::new(*next_id,
                            1.0,
                            1.0,
                            entity.coords.translated(0.0, PROJECTILE_SPAWN_OFFSET, 0.0),
                            EntityType::FlyingBallLinear,
                            entity.team.clone(),
                            IsDummy::True,
                            entity.direction,
                            RANGED_LINEAR_LIFETIME,
                            0,
                            HasGravity::False,
                            HasAI::False,
                            Some(entity.id));

        *next_id += 1;

        dummy.as_mods.push(modifier!(additive, entity.current_weapon.range, 0));
    }

    entities.push(dummy);

    0
}

/// Represents an attack with a RangedProjectile weapon
pub fn attack_ranged_projectile(target_index: usize,
                                entities: &mut Vec<Entity>,
                                next_id: &mut usize)
                                -> usize {
    let mut dummy;

    // Scoped for push call
    {
        let entity = &entities[target_index];

        assert!(entity.current_weapon.weapon_type == WeaponType::RangedProjectile,
                "Can only call attack_ranged_projectile with a
                weapon of type Projectile");

        dummy = Entity::new(*next_id,
                            1.0,
                            1.0,
                            entity.coords.translated(0.0, PROJECTILE_SPAWN_OFFSET, 0.0),
                            EntityType::FlyingBallArc,
                            entity.team.clone(),
                            IsDummy::True,
                            entity.direction,
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

/// Attempts to make the target entity attack
/// Returns the gold gained from killing entities with the attack
/// Do not call this function direction, instead set the entity's `attack` field to true
///
/// Calling this function twice within a single tick can cause the entity to attack twice
/// Setting the `attack` field instead will prevent this
pub fn try_attack(id: usize,
                  entities: &mut Vec<Entity>,
                  next_id: &mut usize,
                  player: &mut Player)
                  -> usize {

    // Get the index of the entity
    let index = entities.iter()
        .enumerate()
        .find(|&(_, e)| e.id == id)
        .unwrap_or_else(|| crash!("Entity not found: {}", id))
        .0;
    let weapon_type;
    let is_attacking;

    // Scoped for attack function calls
    {
        let entity = &mut entities[index];
        weapon_type = entity.current_weapon.weapon_type;
        is_attacking = entity.animations.is_playing(0);

        if !(entity.animations.can_attack() && !is_attacking) && !is_attacking {
            return 0;
        }

        // Calculate animation times
        let attack_time = entity.current_weapon.get_attack_time(&entity.as_mods);
        let pre = entity.current_weapon.anim_pre as f64 * attack_time;
        let post = entity.current_weapon.anim_post as f64 * attack_time;

        // Start the attack animation
        entity.animations.start(0, pre as usize, post as usize);
    }

    // If the pre-animation is finished, attack
    if is_attacking {
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
