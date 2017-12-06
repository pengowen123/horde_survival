//! Common components

use specs;
use cgmath::{self, Rotation3, InnerSpace};

#[derive(Clone, Copy, Debug)]
pub struct Position(pub cgmath::Point3<::Float>);

#[derive(Clone, Copy, Debug)]
pub struct Direction(pub cgmath::Quaternion<::Float>);

impl From<Direction> for cgmath::Vector3<::Float> {
    /// Converts this `Direction` to a direction vector
    fn from(val: Direction) -> cgmath::Vector3<::Float> {
        val.0 * cgmath::Vector3::unit_z()
    }
}

impl From<cgmath::Vector3<::Float>> for Direction {
    /// Converts the direction vector into a `Direction`
    fn from(val: cgmath::Vector3<::Float>) -> Self {
        let val = val.normalize();

        let quat = cgmath::Quaternion::from_arc(cgmath::Vector3::unit_z(), val, None);

        Direction(quat)
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction(cgmath::Quaternion::from_angle_y(cgmath::Deg(0.0)))
    }
}

impl specs::Component for Position {
    type Storage = specs::DenseVecStorage<Self>;
}

impl specs::Component for Direction {
    type Storage = specs::DenseVecStorage<Self>;
}
