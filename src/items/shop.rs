use items::Item;

pub struct ShopItem {
    pub item: &'static Item,
    pub price: usize,
}

impl ShopItem {
    pub const fn new(item: &'static Item, price: usize) -> ShopItem {
        ShopItem {
            item: item,
            price: price,
        }
    }
}
