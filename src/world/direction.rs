use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub struct Direction(pub f64);

impl Direction {
    pub fn as_radians(self) -> f64 {
        self.0 * (PI / 180.0)
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

pub fn get_degrees(radians: f64) -> f64 {
    radians * (180.0 / PI)
}

// Returns an angle, 0 to 180
pub fn get_angle(rise: f64, run: f64) -> f64 {
    let hypotenuse = (rise.powi(2) + run.powi(2)).sqrt();
    let angle = get_degrees((run / hypotenuse).asin());

    if rise < 0.0 {
        return 90.0 + (90.0 - angle);
    } else {
        return angle;
    }
}

// Returns an angle, 0 to 360
pub fn get_angle2(dx: f64, dy: f64) -> f64 {
    let mut angle = get_degrees(dx.atan2(dy));

    angle = if angle < 0.0 {
        180.0 + (180.0 - angle.abs())
    } else {
        angle
    };

    // Rotate angle 180 degrees, because it is always backwards
    Direction(angle + 180.0).wrap().0
}
