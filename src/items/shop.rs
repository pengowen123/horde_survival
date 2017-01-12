use items::Item;

/// An item sold in the shop
pub struct ShopItem {
    pub item: &'static Item,
    pub price: usize,
}
