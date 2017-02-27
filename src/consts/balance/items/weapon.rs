//! All weapon items, represented as constants

use items::*;

weapons!(
    // Test items
    TEST_SWORD, SHOP_TEST_SWORD, 0,
        weapon!("Test Sword",
                25.0, 0.6, 1.0,
                WeaponType::MeleeArea,
                time!(0.5), time!(0.2),
                None);

    TEST_BOW, SHOP_TEST_BOW, 0,
        weapon!("Test Bow",
                60.0, 6.0, 0.8,
                WeaponType::RangedProjectile,
                time!(0.3), time!(0.15),
                None);

    TEST_WAND, SHOP_TEST_WAND, 0,
        weapon!("Test Wand",
                45.0, 3.0, 1.2,
                WeaponType::RangedLinear,
                time!(0.5), time!(0.2),
                None);

    TEST_GUN, SHOP_TEST_GUN, 0,
        weapon!("Test Gun",
                20.0, 100.0, 10.0,
                WeaponType::RangedLinear,
                time!(0.0), time!(0.1),
                None);

    // A placeholder item
    EMPTY, SHOP_EMPTY, 0, weapon!("None", 60.0, 0.4, 1.0, WeaponType::MeleeLine, time!(0.3), time!(0.15), None);

    // Unarmed
    UNARMED, SHOP_UNARMED, 0, weapon!("Unarmed", 60.0, 0.4, 1.0, WeaponType::MeleeLine, time!(0.3), time!(0.15), None);

    LIGHTNING_SWORD_1, SHOP_LIGHTNING_SWORD_1, 2500,
        weapon!("Lightning Sword 2",
                0.0, 6.0, 1.0,
                WeaponType::MeleeLine,
                time!(0.4), time!(0.3),
                Some(&weapon_effect_instant_kill_1))
);
