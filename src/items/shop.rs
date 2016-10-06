use items::Item;

pub struct ShopItem {
    pub item: &'static Item,
    pub price: usize,
}
