use items::*;

pub const WEAPON_UNARMED: Weapon = Weapon::new(15.0, 0.4, 1.5, WeaponType::MeleeLine);

pub const WEAPON_TEST_SWORD: Weapon = Weapon::new(30.0, 1.6, 1.0, WeaponType::MeleeArea);
pub const WEAPON_TEST_BOW: Weapon = Weapon::new(60.0, 0.1, 0.8, WeaponType::RangedProjectile);
pub const WEAPON_TEST_WAND: Weapon = Weapon::new(45.0, 0.05, 1.4, WeaponType::RangedLinear);
