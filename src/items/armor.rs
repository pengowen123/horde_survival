use items::effects::*;

#[derive(Clone, Copy)]
pub struct Armor {
    pub name: &'static str,
    pub multiplier: f64,
    pub when_hit: Option<ItemEffect>,
    pub slot: ArmorSlot,
}

impl Armor {
    pub const fn new(name: &'static str, multiplier: f64, when_hit: Option<ItemEffect>, slot: ArmorSlot) -> Armor {
        Armor {
            name: name,
            multiplier: multiplier,
            when_hit: when_hit,
            slot: slot,
        }
    }
}

impl Armor {
    pub fn is_none(&self) -> bool {
        self.name == "None"
    }
}

// NOTE: This enum is cast to a usize to be used to index the armor field of entities.
//       Because of this, the order of the variants matters.
#[derive(Clone, Copy)]
pub enum ArmorSlot {
    Head,
    Body,
    Legs,
    Feet,
}
