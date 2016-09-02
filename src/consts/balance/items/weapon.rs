use items::*;

pub const WEAPON_UNARMED: Weapon = Weapon::new("None", 15.0, 6.0, 1.5, WeaponType::MeleeLine, None);

pub const WEAPON_TEST_SWORD: Weapon = Weapon::new("Test Sword", 25.0, 0.8, 1.0, WeaponType::MeleeArea, None);
pub const WEAPON_TEST_BOW: Weapon = Weapon::new("Test Bow", 60.0, 1.5, 0.8, WeaponType::RangedProjectile, None);
pub const WEAPON_TEST_WAND: Weapon = Weapon::new("Test Wand", 45.0, 0.05, 1.2, WeaponType::RangedLinear, None);

pub const WEAPON_LIGHTNING_SWORD_2: Weapon = Weapon::new("Lightning Sword 2", 0.0, 6.0, 1.0, WeaponType::MeleeLine, Some(&weapon_effect_instant_kill_10));
