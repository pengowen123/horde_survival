use items::Item;
use consts::items::armor::*;
use consts::items::weapon::*;

use std::collections::HashMap;

pub const TPS: u64 = 30;

pub const fn time(seconds: f64) -> usize { (seconds * TPS as f64) as usize }

pub const BASE_INVENTORY: [Item; 5] = [
    Item::Armor(ARMOR_HEAD_NONE),
    Item::Armor(ARMOR_BODY_NONE),
    Item::Armor(ARMOR_LEGS_NONE),
    Item::Armor(ARMOR_FEET_NONE),
    Item::Weapon(WEAPON_UNARMED)
];

pub fn base_inventory() -> HashMap<usize, Item> {
    let mut inventory = HashMap::new();

    for (i, item) in BASE_INVENTORY.iter().cloned().enumerate() {
        inventory.insert(i, item);
    }

    inventory
}

pub const CRASH_MESSAGE: &'static str = "Horde Survival has crashed. See log for more details.";
