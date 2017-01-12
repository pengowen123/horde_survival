//! Item effects for weapon items

use rand::{Rng, thread_rng};

use consts::*;
use entity::*;
use player::Player;

/// Effect for the Lightning Sword 1
/// Chance to kill the target entity
pub fn weapon_effect_instant_kill_1(_: usize,
                                    being_hit: usize,
                                    entities: &mut Vec<Entity>,
                                    player: &mut Player) {

    if thread_rng().gen_range(0, WEAPON_EFFECT_LIGHTNING_SWORD_2_RANGE) == 0 {
        let entity = &mut entities[being_hit];

        // FIXME: If an entity such as a zombie uses this weapon and procs the effect, the player
        // will gain gold
        player.give_gold(entity.bounty);
        entity.kill();
    }
}
