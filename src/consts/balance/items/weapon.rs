use items::*;

weapons!(
    // Test items
    TEST_SWORD, SHOP_TEST_SWORD, 0, Weapon::new("Test Sword", 25.0, 0.8, 1.0, WeaponType::MeleeArea, None);
    TEST_BOW, SHOP_TEST_BOW, 0, Weapon::new("Test Bow", 60.0, 6.0, 0.8, WeaponType::RangedProjectile, None);
    TEST_WAND, SHOP_TEST_WAND, 0, Weapon::new("Test Wand", 45.0, 1.5, 1.2, WeaponType::RangedLinear, None);

    UNARMED, SHOP_UNARMED, 0, Weapon::new("Unarmed", 15.0, 3.0, 1.0, WeaponType::MeleeLine, None);

    LIGHTNING_SWORD_2, SHOP_LIGHTNING_SWORD_2, 2500, Weapon::new("Lightning Sword 2", 0.0, 6.0, 1.0, WeaponType::MeleeLine, Some(&weapon_effect_instant_kill_10))
);
