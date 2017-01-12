use std::f64::consts::PI;

/// A direction, 0 to 360 degrees
#[derive(Clone, Copy)]
pub struct Direction(pub f64);

impl Direction {
    /// Returns the direction as radians
    pub fn as_radians(self) -> f64 {
        self.0 * (PI / 180.0)
    }

    /// Wraps the direction in the range 0 to 360 degrees
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

/// Returns the radians as degrees
pub fn get_degrees(radians: f64) -> f64 {
    radians * (180.0 / PI)
}

/// Returns the direction moved in, given a rise and a run
/// The angle returned is in the range 0 to 180 degrees (negative movements are treated as positive)

// TODO: Find out how this works to write better docs
pub fn get_angle(rise: f64, run: f64) -> f64 {
    let hypotenuse = (rise.powi(2) + run.powi(2)).sqrt();
    let mut angle = get_degrees((run / hypotenuse).asin());

    angle = if rise > 0.0 {
        90.0 + (90.0 - angle)
    } else {
        angle
    };

    if angle.is_nan() {
        warn!("get_angle returned NAN");
        0.0
    } else {
        angle
    }
}

/// Returns the direction moved in, given a rise and a run
/// The angle returned is in the range 0 to 360 degrees (negative movements return an angle between
/// 180 and 360 degrees)

// TODO: Find out how this works to write better docs
pub fn get_angle2(dx: f64, dy: f64) -> f64 {
    let mut angle = get_degrees(dx.atan2(dy));

    angle = if angle < 0.0 {
        180.0 + (180.0 - angle.abs())
    } else {
        angle
    };

    if angle.is_nan() {
        warn!("get_angle2 returned NAN");
        0.0
    } else {
        // Rotate angle 180 degrees, because it is always backwards
        Direction(angle + 180.0).wrap().0
    }
}
