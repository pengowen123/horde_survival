//! Functions for player abilities

// TODO: Move each class's ability functions into a new abilities directory for better organization

use entity::*;
use player::Player;
use consts::balance::*;

pub type Ability = fn(&mut Player, &mut Vec<Entity>);

/// Ability 0 for the Warrior class
/// Reduces the damage the player takes for a duration (see `WARRIOR_FORTIFY`)
pub fn warrior_ability_0(player: &mut Player, entities: &mut Vec<Entity>) {
    let player_entity = find_player_entity!(entities.iter_mut(), player);

    player_entity.damage_taken_mods.push(WARRIOR_FORTIFY);
}

/// Ability 1 for the Warrior class
/// Reduces the damage and movespeed of nearby entities for a duration (see `WARRIOR_MAIM_*`)
pub fn warrior_ability_1(player: &mut Player, entities: &mut Vec<Entity>) {
    let coords = find_player_entity!(entities.iter(), player).coords;

    for entity in entities.iter_mut()
        .filter(|e| e.team != Team::Players && e.coords.in_radius(&coords, WARRIOR_MAIM_RADIUS)) {
        entity.damage_mods.push(WARRIOR_MAIM_DAMAGE);
        entity.movespeed_mods.push(WARRIOR_MAIM_SLOW);
    }
}

/// Ability 2 for the Warrior class
/// Increases the attack speed and damage of the player for a duration (see `WARRIOR_RAGE_*`)
pub fn warrior_ability_2(player: &mut Player, entities: &mut Vec<Entity>) {
    let player_entity = find_player_entity!(entities.iter_mut(), player);

    player_entity.as_mods.push(WARRIOR_RAGE_AS);
    player_entity.damage_mods.push(WARRIOR_RAGE_DAMAGE);
}

/// Ability 3 for the Warrior class
/// Deals damage to nearby entities (see `WARRIOR_EXECUTE_*`)
pub fn warrior_ability_3(player: &mut Player, entities: &mut Vec<Entity>) {
    let coords;
    let player_index;

    // Scoped for damage call
    {
        let (i, entity) = find_player_entity!(INDEX, entities.iter(), player);
        coords = entity.coords;
        player_index = i;
    }

    // Get the indices of the entities to be hit by the ability
    let indices = entities.iter()
        .enumerate()
        .filter_map(|(i, e)| {
            if e.team != Team::Players && e.coords.in_radius(&coords, WARRIOR_EXECUTE_RADIUS) {
                Some(i)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for i in indices {
        // This is inefficient, but is necessary to prevent a double mutable borrow
        let mut clone = entities[i].clone();

        // Deal damage to the entity
        clone.damage(WARRIOR_EXECUTE_DAMAGE, i, player_index, entities, player);

        // Replace the original entity with the clone
        let entity = entities.iter_mut()
            .find(|e| e.id == clone.id)
            .unwrap_or_else(|| crash!("Call to damage removed entity"));
        *entity = clone;

        // This is okay because the player is the only one who can cast abilities, but it will have
        // to be changed when multiplayer is added
        if entity.is_dead() {
            player.give_gold(entity.bounty);
        }
    }
}
