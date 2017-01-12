use items::effects::*;

/// An armor item
#[derive(Clone, Copy)]
pub struct Armor {
    pub name: &'static str,
    pub multiplier: f64,
    pub when_hit: Option<ArmorEffect>,
    pub slot: ArmorSlot,
}

impl Armor {
    /// Returns whether the armor slot is empty
    pub fn is_empty(&self) -> bool {
        self.name == "None"
    }
}

// NOTE: This enum is cast to a usize to be used to index the armor field of entities
/// The slot an armor item can be used in
#[derive(Clone, Copy)]
pub enum ArmorSlot {
    Head = 0,
    Body = 1,
    Legs = 2,
    Feet = 3,
}
