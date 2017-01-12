use hscontrols::movement;

/// The state of the player's inputs
pub struct Input {
    pub forward: bool,
    pub left: bool,
    pub right: bool,
    pub back: bool,
}

impl Input {
    /// Returns whether a movement key is being pressed
    pub fn movement_key_pressed(&self) -> bool {
        self.forward || self.left || self.right || self.back
    }

    /// Returns whether the offset angle of player movement (see get_movement_offset)
    ///
    /// Only call this function if Input::movement_key_pressed returns true
    pub fn movement_offset(&self) -> f64 {
        assert!(self.movement_key_pressed());
        movement::get_movement_offset(self.forward, self.left, self.right, self.back)
    }
}

impl Default for Input {
    fn default() -> Self {
        Input {
            forward: false,
            left: false,
            right: false,
            back: false,
        }
    }
}
