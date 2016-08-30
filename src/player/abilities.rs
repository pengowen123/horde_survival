use entity::*;
use player::Player;
use consts::balance::*;

pub fn warrior_ability_0(player: &mut Player, entities: &mut Vec<Entity>) {
    let player_entity = entities.iter_mut().find(|e| e.id == player.entity_id).expect("Player entity not found");

    player_entity.damage_taken_mods.push(WARRIOR_FORTIFY);
}

pub fn warrior_ability_1(player: &mut Player, entities: &mut Vec<Entity>) {
    let coords = entities.iter().find(|e| e.id == player.entity_id)
        .expect("Player entity not found")
        .coords.clone();
    
    for entity in entities.iter_mut().filter(|e| e.is_enemy && e.coords.in_radius(&coords, WARRIOR_MAIM_RADIUS)) {
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
    let coords = entities.iter().find(|e| e.id == player.entity_id)
        .expect("Player entity not found")
        .coords.clone();

    for entity in entities.iter_mut().filter(|e| e.is_enemy && e.coords.in_radius(&coords, WARRIOR_EXECUTE_RADIUS)) {
        entity.damage(WARRIOR_EXECUTE_DAMAGE);

        if entity.is_dead() {
            player.gold += get_bounty(player.wave);
        }
    }
}
