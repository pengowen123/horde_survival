//! Player inventory handling

use entity::Entity;
use player::Player;
use items::*;

impl Player {
    /// Equips an item from the player's inventory
    pub fn equip_from_inventory(&mut self, id: usize, entity: &mut Entity) {
        let item = self.inventory.get(&id).unwrap_or_else(|| crash!("Item not found: {}", id));

        match *item {
            Item::Weapon(ref weapon) => entity.current_weapon = weapon.clone(),
            Item::Armor(ref armor) => entity.armor[armor.slot as i32 as usize] = *armor,
        }
    }

    /// Adds the item to the player's inventory
    pub fn give_item(&mut self, item: Item) {
        let new_id = self.inventory.len();
        self.inventory.insert(new_id, item);
    }

    /// Adds the item to the player's inventory if they have enough gold to buy it
    /// If so, the player's gold is reduced by the item's price
    /// Returns true if the item was added to the player's inventory
    pub fn buy_item(&mut self, item: &ShopItem) -> bool {
        if self.gold >= item.price {
            self.give_item(item.item.clone());
            self.gold -= item.price;
            true
        } else {
            false
        }
    }
}
