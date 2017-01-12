use random_choice::random_choice;

use consts::*;
use items::effects::*;
use entity::modifiers::{Modifier, apply};

use std::usize;

/// A weapon item
#[derive(Clone)]
pub struct Weapon {
    pub name: &'static str,
    pub damage: f64,
    pub range: f64,
    pub attack_speed: f64,
    pub anim_pre: usize,
    pub anim_post: usize,
    pub weapon_type: WeaponType,
    pub on_hit: Option<WeaponEffect>,
}

/// The type of a weapon
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponType {
    MeleeLine,
    MeleeArea,
    RangedLinear,
    RangedProjectile,
}

impl Weapon {
    /// Returns the actual range of the weapon, measured by distance
    /// For RangedProjectile weapons, the range is an estimate
    pub fn get_real_range(&self) -> f64 {
        match self.weapon_type {
            WeaponType::MeleeLine => self.range * MELEE_LINE_INTERVAL * MELEE_LINE_RADIUS * 2.0,
            WeaponType::MeleeArea => self.range * 2.0,
            WeaponType::RangedLinear => {
                ((self.range + 1.0) * RANGED_INTERVAL * RANGED_LINEAR_LIFETIME as f64).powf(0.75)
            }
            WeaponType::RangedProjectile => (self.range * (0.1 / GRAVITY)).powf(0.4),
        }
    }

    /// Returns the attack time of the weapon, with the given modifiers applied
    /// The returned value is not how long it takes for the weapon's animation to finish, but
    /// instead is used to calculate animation time
    pub fn get_attack_time(&self, modifiers: &[Modifier]) -> f64 {
        let attack_speed = apply(modifiers, self.attack_speed);
        let x = 1.0 / attack_speed;

        x * GLOBAL_ATTACK_TIME
    }
}

/// Returns a random weapon to be used by a monster, based on the current wave
pub fn get_random_monster_weapon(wave: usize) -> Weapon {
    // TODO: Replace these with a const containing actual items to be used by monsters
    let items = [UNARMED, TEST_SWORD, TEST_WAND, TEST_BOW, TEST_GUN];
    let weights = [1.0, 1.0, 1.0, 1.0, 1.0];
    let mut rng = random_choice();

    let range = match wave {
        9...11 => ..4,
        6...8 => ..3,
        3...5 => ..2,
        0...2 => ..1,
        _ => ..items.len(),
    };

    rng.random_choice_f64(&items[range], &weights[range], 1)[0].clone()
}
