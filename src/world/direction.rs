use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub struct Direction(pub f64);

impl Direction {
    pub fn as_radians(self) -> f64 {
        self.0 * (PI/ 180.0)
    }

    pub fn wrap(mut self) -> Direction {
        while self.0 >= 360.0 {
            self.0 -= 360.0;
        }

        while self.0 < 0.0 {
            self.0 += 360.0;
        }

        self
    }
}
