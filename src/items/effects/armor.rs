use rand::{Rng, thread_rng};

use consts::balance::items::effects::*;
use entity::*;
use player::Player;

pub fn armor_effect_heal(entity_index: usize, hit_by: usize, entities: &mut Vec<Entity>, player: &mut Player) {
    if thread_rng().gen_range(0, ARMOR_EFFECT_HEAL_RANGE) == 0 {
        entities[entity_index].heal(ARMOR_EFFECT_HEAL_AMOUNT);
    }
}
