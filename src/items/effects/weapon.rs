use rand::{Rng, thread_rng};

use consts::*;
use entity::*;
use player::Player;

pub fn weapon_effect_instant_kill_10(entity_index: usize, being_hit: usize, entities: &mut Vec<Entity>, player: &mut Player) {
    if thread_rng().gen_range(0, WEAPON_EFFECT_LIGHTNING_SWORD_2_RANGE) == 0 {
        let entity = &mut entities[being_hit];

        player.give_gold(entity.bounty);
        entity.kill();
    }
}
