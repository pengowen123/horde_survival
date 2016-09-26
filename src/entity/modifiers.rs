// NOTE: modifiers expire when their timer reaches 1
//       modifier are permanent if their timer is set to 0
#[derive(Clone, Debug)]
pub struct Modifier {
    pub value: f64,
    pub timer: usize,
    pub kind: ModifierKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModifierKind {
    // Adds to the base value, calculated first
    Additive,
    // Adds to the multiplier, calculated second
    Multiplier,
    // Multiplies the multiplier, calculated last
    Multiplicative,
}

impl Modifier {
    pub const fn new(value: f64, timer: usize, kind: ModifierKind) -> Modifier {
        Modifier {
            value: value,
            timer: timer,
            kind: kind,
        }
    }

    pub const fn additive(value: f64, timer: usize) -> Modifier {
        Modifier::new(value, timer, ModifierKind::Additive)
    }

    pub const fn multiplier(value: f64, timer: usize) -> Modifier {
        Modifier::new(value, timer, ModifierKind::Multiplier)
    }

    pub const fn multiplicative(value: f64, timer: usize) -> Modifier {
        Modifier::new(value, timer, ModifierKind::Multiplicative)
    }

    pub fn update(&mut self) {
        if self.timer > 1 {
            self.timer -= 1;
        }
    }

    pub fn is_expired(&self) -> bool {
        self.timer == 1
    }
}

pub fn apply(mods: &[Modifier], mut base: f64) -> f64 {
    let mut multiplier = 1.0;

    for modifier in mods.iter().filter(|m| m.kind == ModifierKind::Additive) {
        base += modifier.value;
    }

    for modifier in mods.iter().filter(|m| m.kind == ModifierKind::Multiplier) {
        multiplier += modifier.value;
    }

    for modifier in mods.iter().filter(|m| m.kind == ModifierKind::Multiplicative) {
        multiplier *= modifier.value;
    }

    base * multiplier
}
