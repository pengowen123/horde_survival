#[derive(Clone, Debug)]
pub struct Modifier {
    pub value: f64,
    pub timer: usize,
}

impl Modifier {
    pub const fn new(value: f64, timer: usize) -> Modifier {
        Modifier {
            value: value,
            timer: timer
        }
    }

    pub fn update(&mut self) {
        if self.timer > 0 {
            self.timer -= 1;
        }
    }

    pub fn is_expired(&self) -> bool {
        self.timer == 0
    }
}
