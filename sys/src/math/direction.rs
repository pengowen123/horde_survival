//! A simple direction type and functions for manipulating them

use cgmath::{self, Rad};

use std::ops;

use math::functions;
use math::consts::{PI, TAU};

/// An angle, in radians.
pub type Angle = Rad<::Float>;

/// A type representing a direction in a 3D space
///
/// Pitch ranges from `0.0` to `PI` radians (straight up and down, respectively).
/// Yaw ranges from `0.0` to `PI * 2.0` radians (increasing yaw rotates counter-clockwise).
#[derive(Clone, Copy, Debug)]
pub struct Direction {
    /// Up and down
    pitch: Angle,
    /// Left and right
    yaw: Angle,
}

impl Direction {
    pub fn new<T: Into<Rad<::Float>>>(pitch: T, yaw: T) -> Self {
        let pitch = pitch.into();
        let yaw = yaw.into();

        Self { pitch, yaw }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::new(Rad(0.0), Rad(0.0))
    }
}

/// Converts a direction to a unit vector pointing in that direction
impl Into<cgmath::Vector3<::Float>> for Direction {
    fn into(self) -> cgmath::Vector3<::Float> {
        use cgmath::Angle;

        let yaw = self.yaw;
        let pitch = self.pitch;

        let x = yaw.cos() * pitch.sin();
        let z = pitch.cos();
        let y = yaw.sin() * pitch.sin();

        cgmath::Vector3::new(x, y, z)
    }
}

impl ops::Add for Direction {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        // When adding Direction's, only allow the pitch to be from -1 to 1 radian (straight down and up)
        // Also make the yaw loop around instead of capping like with pitch

        let mut pitch = self.pitch + other.pitch;
        let mut yaw = self.yaw + other.yaw;

        // Set the min and max pitch to straight down and straight up
        pitch = functions::clamp(pitch, Rad(0.0), Rad(PI));
        // Allow the yaw to loop around
        yaw = functions::wrap(yaw, Rad(0.0), Rad(TAU));

        Direction { pitch, yaw }
    }
}

impl ops::AddAssign for Direction {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cgmath::Vector3;

    #[test]
    fn test_direction_into_vector() {
        let direction = Direction::new(Deg(45.0), Deg(45.0));
        //assert_eq!(direction.into_vector(), Vector3::new())
    }
}
