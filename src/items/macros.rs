/// Creates a Weapon and a ShopItem for each set of values
macro_rules! weapons {
    ($($name:ident, $shop_name:ident, $price:expr, $item:expr);*) => {
        $(
            #[allow(dead_code)]
            pub const $name: $crate::items::weapon::Weapon = $item;
            #[allow(dead_code)]
            pub const $shop_name: $crate::items::shop::ShopItem = shop_item!(&$crate::items::Item::Weapon($name), $price);
        )*
    }
}

/// Creates an Armor and a ShopItem for each set of values
macro_rules! armors {
    ($($name:ident, $shop_name:ident, $price:expr, $item:expr);*) => {
        $(
            #[allow(dead_code)]
            pub const $name: $crate::items::armor::Armor = $item;
            #[allow(dead_code)]
            pub const $shop_name: $crate::items::shop::ShopItem = shop_item!(&$crate::items::Item::Armor($name), $price);
        )*
    }
}

/// Creates a Weapon
macro_rules! weapon {
    ($name:expr, $damage:expr, $range:expr, $attack_speed:expr, $weapon_type:expr, $anim_pre:expr, $anim_post:expr, $on_hit:expr) => {{
        $crate::items::weapon::Weapon {
            name: $name,
            damage: $damage,
            range: $range,
            attack_speed: $attack_speed,
            weapon_type: $weapon_type,
            anim_pre: $anim_pre,
            anim_post: $anim_post,
            on_hit: $on_hit,
        }
    }}
}

/// Creates an Armor
macro_rules! armor {
    ($name:expr, $multiplier:expr, $when_hit:expr, $slot:expr) => {{
        $crate::items::armor::Armor {
            name: $name,
            multiplier: $multiplier,
            when_hit: $when_hit,
            slot: $slot,
        }
    }}
}

/// Creates a shop item
macro_rules! shop_item {
    ($item:expr, $price:expr) => {{
        $crate::items::shop::ShopItem {
            item: $item,
            price: $price,
        }
    }}
}
