use hscontrols::movement;

pub struct Input {
    pub forward: bool,
    pub left: bool,
    pub right: bool,
    pub back: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            forward: false,
            left: false,
            right: false,
            back: false,
        }
    }

    pub fn movement_key_pressed(&self) -> bool {
        self.forward || self.left || self.right || self.back
    }

    pub fn movement_offset(&self) -> f64 {
        movement::get_movement_offset(self.forward, self.left, self.right, self.back)
    }
}
