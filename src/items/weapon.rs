// NOTE: display attack range of MeleeLine weapons as range * interval * radius * 2
//       display attack range of MeleeArea weapons as range * 2
//       display attack range of RangedLinear weapons as range * projectile_lifetime
//       don't display attack range of RangedProjectile weapons (use range as projectile speed)

use consts::*;
use items::effects::*;

#[derive(Clone)]
pub struct Weapon {
    pub name: &'static str,
    pub damage: f64,
    pub range: f64,
    pub attack_speed: f64,
    pub weapon_type: WeaponType,
    pub on_hit: Option<ItemEffect>,
}

#[derive(Clone)]
pub enum WeaponType {
    MeleeLine,
    MeleeArea,
    RangedLinear,
    RangedProjectile,
}

impl Weapon {
    pub const fn new(name: &'static str, damage: f64, range: f64, attack_speed: f64, weapon_type: WeaponType, on_hit: Option<ItemEffect>) -> Weapon {
        Weapon {
            name: name,
            damage: damage,
            range: range,
            attack_speed: attack_speed,
            weapon_type: weapon_type,
            on_hit: on_hit,
        }
    }
}

impl Weapon {
    pub fn get_real_range(&self) -> f64 {
        match self.weapon_type {
            WeaponType::MeleeLine => self.range * MELEE_LINE_INTERVAL * MELEE_LINE_RADIUS * 2.0,
            WeaponType::MeleeArea => self.range * 2.0,
            WeaponType::RangedLinear => ((self.range + 1.0) * RANGED_INTERVAL * RANGED_LINEAR_LIFETIME as f64).powf(0.75),
            WeaponType::RangedProjectile => (self.range * 3.0).sqrt(),
        }
    }

    pub fn get_attack_time(&self, attack_speed: f64) -> usize {
        let x = 1.0 / attack_speed;

        time(x * GLOBAL_ATTACK_TIME)
    }
}
