//! Item effects for armor items

use rand::{Rng, thread_rng};

use consts::balance::items::effects::*;
use entity::*;
use player::Player;

/// Effect for the Heal armor
/// Chance to heal the entity being hit
pub fn armor_effect_heal(_: usize,
                         entity_index: usize,
                         entities: &mut Vec<Entity>,
                         _: &mut Player) {

    if thread_rng().gen_range(0, ARMOR_EFFECT_HEAL_RANGE) == 0 {
        entities[entity_index].heal(ARMOR_EFFECT_HEAL_AMOUNT);
    }
}
