use items::*;
use consts::misc::time;

weapons!(
    // Test items
    TEST_SWORD, SHOP_TEST_SWORD, 0, Weapon::new("Test Sword", 25.0, 0.6, 1.0, WeaponType::MeleeArea, time(0.5), time(0.2), None);
    TEST_BOW, SHOP_TEST_BOW, 0, Weapon::new("Test Bow", 60.0, 6.0, 0.8, WeaponType::RangedProjectile, time(0.3), time(0.15), None);
    TEST_WAND, SHOP_TEST_WAND, 0, Weapon::new("Test Wand", 45.0, 3.0, 1.2, WeaponType::RangedLinear, time(0.5), time(0.2), None);
    TEST_GUN, SHOP_TEST_GUN, 0, Weapon::new("Test Gun", 20.0, 100.0, 10.0, WeaponType::RangedLinear, time(0.0), time(0.1), None);

    UNARMED, SHOP_UNARMED, 0, Weapon::new("Unarmed", 60.0, 0.4, 1.0, WeaponType::MeleeLine, time(0.3), time(0.15), None);

    LIGHTNING_SWORD_2, SHOP_LIGHTNING_SWORD_2, 2500, Weapon::new("Lightning Sword 2", 0.0, 6.0, 1.0, WeaponType::MeleeLine, time(0.4), time(0.3), Some(&weapon_effect_instant_kill_10))
);
