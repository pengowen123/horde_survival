use utils::*;
use entity::*;
use player::Player;

// Cooldown constants

pub const WARRIOR_COOLDOWN_0: usize = time(7.0);
pub const WARRIOR_COOLDOWN_1: usize = time(8.0);
pub const WARRIOR_COOLDOWN_2: usize = time(10.0);
pub const WARRIOR_COOLDOWN_3: usize = time(12.0);

// Warrior ability constants
const WARRIOR_FORTIFY: Modifier = Modifier::new(0.5, time(3.0));

const WARRIOR_MAIM_DAMAGE: Modifier = Modifier::new(0.65, time(4.0));
const WARRIOR_MAIM_SLOW: Modifier = Modifier::new(0.65, time(4.0));
const WARRIOR_MAIM_RADIUS: f64 = 3.0;

const WARRIOR_RAGE_AS: Modifier = Modifier::new(1.3, time(4.0));
const WARRIOR_RAGE_DAMAGE: Modifier = Modifier::new(1.3, time(4.0));

const WARRIOR_EXECUTE_DAMAGE: f64 = 120.0;
const WARRIOR_EXECUTE_RADIUS: f64 = 2.0;

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
