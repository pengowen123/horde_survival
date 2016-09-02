pub mod weapon;
pub mod armor;
pub mod effects;
pub mod shop;

pub use self::weapon::*;
pub use self::armor::*;
pub use self::effects::weapon::*;
pub use self::effects::armor::*;
pub use self::shop::*;

#[derive(Clone)]
pub enum Item {
    Weapon(Weapon),
    Armor(Armor),
}
