// NOTE: modifiers expire when their timer reaches 1
//       modifier are permanent if their timer is set to 0
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
        if self.timer > 1 {
            self.timer -= 1;
        }
    }

    pub fn is_expired(&self) -> bool {
        self.timer == 1
    }
}
