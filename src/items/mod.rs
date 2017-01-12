//! Items used by entities

pub mod weapon;
pub mod armor;
pub mod effects;
pub mod shop;
#[macro_use]
pub mod macros;

pub use self::weapon::*;
pub use self::armor::*;
pub use self::effects::weapon::*;
pub use self::effects::armor::*;
pub use self::shop::*;

/// Every possible item type
#[derive(Clone)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
}
