//! Types for storing input state

use common::cgmath;

bitflags! {
    /// A type that stores movement key input state
    #[derive(Default)]
    pub struct InputState: u8 {
        const FORWARD =  1;
        const RIGHT =    1 << 1;
        const BACKWARD = 1 << 2;
        const LEFT =     1 << 3;
    }
}

impl InputState {
    /// Returns an angle to offset the movement direction by
    ///
    /// The angle represents a counter-clockwise rotation
    pub fn get_movement_angle(&self) -> Option<cgmath::Rad<::Float>> {
        let f = self.contains(InputState::FORWARD);
        let r = self.contains(InputState::RIGHT);
        let b = self.contains(InputState::BACKWARD);
        let l = self.contains(InputState::LEFT);

        let angle = match (f, r, b, l) {
            // If all keys or no keys are pressed, don't move
            (true, true, true, true) => None,
            (false, false, false, false) => None,
            // Forward
            (true, false, false, false) => Some(0.0),
            // Right
            (false, true, false, false) => Some(270.0),
            // Backward
            (false, false, true, false) => Some(180.0),
            // Left
            (false, false, false, true) => Some(90.0),
            // Forward + Right
            (true, true, false, false) => Some(315.0),
            // Right + Backward
            (false, true, true, false) => Some(225.0),
            // Backward + Left
            (false, false, true, true) => Some(135.0),
            // Left + Forward
            (true, false, false, true) => Some(45.0),
            // Forward + Backward
            (true, false, true, false) => None,
            // Right + Left
            (false, true, false, true) => None,
            // Forward + Right + Backward
            (true, true, true, false) => Some(270.0),
            // Forward + Right + Left
            (true, true, false, true) => Some(0.0),
            // Forward + Backward + Left
            (true, false, true, true) => Some(90.0),
            // Right + Backward + Left
            (false, true, true, true) => Some(180.0),
        };

        angle.map(|a| cgmath::Deg(a).into())
    }
}

/// A movement direction
#[repr(u8)]
pub enum Direction {
    Forward = InputState::FORWARD.bits,
    Right = InputState::RIGHT.bits,
    Backward = InputState::BACKWARD.bits,
    Left = InputState::LEFT.bits,
}
