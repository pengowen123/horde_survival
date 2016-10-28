use random_choice::random_choice;

use consts::*;
use items::effects::*;

use std::usize;

#[derive(Clone)]
pub struct Weapon {
    pub name: &'static str,
    pub damage: f64,
    pub range: f64,
    pub attack_speed: f64,
    pub anim_pre: usize,
    pub anim_post: usize,
    pub weapon_type: WeaponType,
    pub on_hit: Option<ItemEffect>,
}

#[derive(Clone, Debug)]
pub enum WeaponType {
    MeleeLine,
    MeleeArea,
    RangedLinear,
    RangedProjectile,
}

impl Weapon {
    pub fn get_real_range(&self) -> f64 {
        match self.weapon_type {
            WeaponType::MeleeLine => self.range * MELEE_LINE_INTERVAL * MELEE_LINE_RADIUS * 2.0,
            WeaponType::MeleeArea => self.range * 2.0,
            WeaponType::RangedLinear => ((self.range + 1.0) * RANGED_INTERVAL * RANGED_LINEAR_LIFETIME as f64).powf(0.75),
            WeaponType::RangedProjectile => (self.range * (0.1 / GRAVITY)).powf(0.4),
        }
    }

    pub fn get_attack_time(&self, attack_speed: f64) -> f64 {
        let x = 1.0 / attack_speed;

        x * GLOBAL_ATTACK_TIME
    }
}

pub fn get_random_monster_weapon(wave: usize) -> Weapon {
    let items = [UNARMED, TEST_SWORD, TEST_WAND, TEST_BOW, TEST_GUN];
    let weights = [1.0, 1.0, 1.0, 1.0, 1.0];
    let mut rng = random_choice();
    // NOTE: using inf causes the compiler to panic, so for now use a high number such as 100
    //let inf = usize::MAX as i32 as f64 as i32 as usize;

    let range = match wave {
        9...11 => ..4,
        6...8 => ..3,
        3...5 => ..2,
        0...2 => ..1,
        _ => ..items.len(),
    };

    rng.random_choice_f64(&items[range], &weights[range], 1)[0].clone()
}
