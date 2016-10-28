use entity::Entity;
use player::Player;
use items::*;

impl Player {
    pub fn equip_from_inventory(&mut self, id: usize, entity: &mut Entity) {
        match self.inventory.get(&id) {
            Some(item) => {
                match *item {
                    Item::Weapon(ref weapon) => entity.current_weapon = weapon.clone(),
                    Item::Armor(ref armor) => entity.armor[armor.slot as i32 as usize] = *armor,
                }
            },
            None => crash!("Item not found: {}", id),
        }
    }

    pub fn give_item(&mut self, item: Item) {
        let new_id = self.inventory.len();
        self.inventory.insert(new_id, item);
    }

    pub fn buy_item(&mut self, item: &ShopItem) -> bool {
        if self.gold >= item.price {
            self.give_item(item.item.clone());
            true
        } else {
            false
        }
    }
}
