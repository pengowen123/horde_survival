use items::*;

armors!(
    HEAD_NONE, SHOP_HEAD_NONE, 0, armor!("None", 1.0, None, ArmorSlot::Head);
    BODY_NONE, SHOP_BODY_NONE, 0, armor!("None", 1.0, None, ArmorSlot::Body);
    LEGS_NONE, SHOP_LEGS_NONE, 0, armor!("None", 1.0, None, ArmorSlot::Legs);
    FEET_NONE, SHOP_FEET_NONE, 0, armor!("None", 1.0, None, ArmorSlot::Feet);

    HEAL, SHOP_HEAL, 1500, armor!("Healing Armor", 0.8, Some(&armor_effect_heal), ArmorSlot::Body)
);
