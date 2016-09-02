use consts::items::armor::*;
use consts::items::weapon::*;
use items::*;

pub const SHOP: &'static [ShopItem] = &[
    ShopItem::new(&Item::Weapon(WEAPON_LIGHTNING_SWORD_2), 2500),
    ShopItem::new(&Item::Armor(ARMOR_HEAL), 1500),
];
