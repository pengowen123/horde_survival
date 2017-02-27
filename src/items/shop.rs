use items::Item;

/// An item sold in the shop
#[derive(Clone)]
pub struct ShopItem {
    pub item: &'static Item,
    pub price: usize,
}

impl ShopItem {
    /// Returns whether the item is a dummy
    pub fn is_dummy(&self) -> bool {
        self.item.is_dummy()
    }
}
