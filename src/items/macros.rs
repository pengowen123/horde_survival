macro_rules! weapons {
    ($($name:ident, $shop_name:ident, $price:expr, $item:expr);*) => {
        $(
            #[allow(dead_code)]
            pub const $name: $crate::items::weapon::Weapon = $item;
            #[allow(dead_code)]
            pub const $shop_name: $crate::items::shop::ShopItem = $crate::items::shop::ShopItem::new(
                &$crate::items::Item::Weapon($name), $price);
        )*
    }
}

macro_rules! armors {
    ($($name:ident, $shop_name:ident, $price:expr, $item:expr);*) => {
        $(
            #[allow(dead_code)]
            pub const $name: $crate::items::armor::Armor = $item;
            #[allow(dead_code)]
            pub const $shop_name: $crate::items::shop::ShopItem = $crate::items::shop::ShopItem::new(
                &$crate::items::Item::Armor($name), $price);
        )*
    }
}
