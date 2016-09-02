use items::*;

pub const ARMOR_HEAD_NONE: Armor = Armor::new("None", 1.0, None, ArmorSlot::Head);
pub const ARMOR_BODY_NONE: Armor = Armor::new("None", 1.0, None, ArmorSlot::Body);
pub const ARMOR_LEGS_NONE: Armor = Armor::new("None", 1.0, None, ArmorSlot::Legs);
pub const ARMOR_FEET_NONE: Armor = Armor::new("None", 1.0, None, ArmorSlot::Feet);

pub const ARMOR_HEAL: Armor = Armor::new("Healing Armor", 0.8, Some(&armor_effect_heal), ArmorSlot::Body);
