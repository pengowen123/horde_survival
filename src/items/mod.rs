// NOTE: display attack radius of MeleeLine weapons as range * interval * radius
//       display attack radius of MeleeArea weapons as range * 2
//       display attack radius of RangedLinear weapons as range * projectile_lifetime
//       don't display attack radius of RangedProjectile weapons

pub mod utils;

pub use self::utils::get_attack_time;

#[derive(Clone)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
    Other(Other),
}

#[derive(Clone)]
pub struct Weapon {
    pub damage: f64,
    pub range: f64,
    pub attack_speed: f64,
    pub weapon_type: WeaponType,
}

#[derive(Clone)]
pub enum WeaponType {
    MeleeLine,
    MeleeArea,
    RangedLinear,
    RangedProjectile,
}

#[derive(Clone)]
pub struct Armor;

#[derive(Clone)]
pub struct Other;

impl Weapon {
    pub const fn new(damage: f64, range: f64, attack_speed: f64, weapon_type: WeaponType) -> Weapon {
        Weapon {
            damage: damage,
            range: range,
            attack_speed: attack_speed,
            weapon_type: weapon_type,
        }
    }
}
