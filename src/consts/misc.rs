use items::Item;
use consts::items::armor::*;
use consts::items::weapon::*;

use std::collections::HashMap;

pub const TPS: u64 = 30;
pub const TPS_FLOAT: f64 = TPS as f64;

pub const BASE_INVENTORY: [Item; 5] = [
    Item::Armor(HEAD_NONE),
    Item::Armor(BODY_NONE),
    Item::Armor(LEGS_NONE),
    Item::Armor(FEET_NONE),
    Item::Weapon(UNARMED)
];

pub fn base_inventory() -> HashMap<usize, Item> {
    let mut inventory = HashMap::new();

    for (i, item) in BASE_INVENTORY.iter().cloned().enumerate() {
        inventory.insert(i, item);
    }

    inventory
}

pub const CRASH_MESSAGE: &'static str = "Horde Survival has crashed";

macro_rules! time {
    ($seconds:expr) => { ($seconds * $crate::consts::TPS_FLOAT) as usize}
}
