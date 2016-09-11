use entity::*;
use player::Player;
use consts::balance::*;
use hslog::*;

pub fn warrior_ability_0(player: &mut Player, entities: &mut Vec<Entity>) {
    let player_entity = entities.iter_mut().find(|e| e.id == player.entity_id).expect("Player entity not found");

    player_entity.damage_taken_mods.push(WARRIOR_FORTIFY);
}

pub fn warrior_ability_1(player: &mut Player, entities: &mut Vec<Entity>) {
    let coords = unwrap_or_log!(entities.iter().find(|e| e.id == player.entity_id),
                                "Player entity not found").coords.clone();

    for entity in entities.iter_mut().filter(|e| e.team != Team::Players && e.coords.in_radius(&coords, WARRIOR_MAIM_RADIUS)) {
        entity.damage_mods.push(WARRIOR_MAIM_DAMAGE);
        entity.movespeed_mods.push(WARRIOR_MAIM_SLOW);
    }
}

pub fn warrior_ability_2(player: &mut Player, entities: &mut Vec<Entity>) {
    let player_entity = entities.iter_mut().find(|e| e.id == player.entity_id).expect("Player entity not found");

    player_entity.as_mods.push(WARRIOR_RAGE_AS);
    player_entity.damage_mods.push(WARRIOR_RAGE_DAMAGE);
}

pub fn warrior_ability_3(player: &mut Player, entities: &mut Vec<Entity>) {
    let coords;
    let player_index;

    // Scoped for damage call
    {
        let (i, entity) = unwrap_or_log!(entities.iter().enumerate().find(|&(_, ref e)| e.id == player.entity_id),
                                         "Player entity not found");

        coords = entity.coords.clone();
        player_index = i;
    }

    let indices = entities.iter().enumerate().filter_map(|(i, ref e)| {
        if e.team != Team::Players && e.coords.in_radius(&coords, WARRIOR_EXECUTE_RADIUS) {
            Some(i)
        } else {
            None
        }
    }).collect::<Vec<_>>();

    for i in indices {
        // This is inefficient, but is necessary to prevent bad things from happening
        let mut clone = entities[i].clone();

        clone.damage(WARRIOR_EXECUTE_DAMAGE, i, player_index, entities, player);

        let entity = unwrap_or_log!(entities.iter_mut().find(|e| e.id == clone.id),
                                    "Call to damage removed entity");

        *entity = clone;

        if entity.is_dead() {
            player.give_bounty(1);
        }
    }
}
