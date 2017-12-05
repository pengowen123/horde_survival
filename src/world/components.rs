//! Components related to the world

use specs;
use cgmath::{self, Rotation3};

pub type Position = cgmath::Point3<::Float>;

#[derive(Clone, Copy, Debug)]
pub struct Spatial(pub Position);

#[derive(Clone, Copy, Debug)]
pub struct Direction(pub cgmath::Quaternion<::Float>);

impl Direction {
    pub fn into_vector(self) -> cgmath::Vector3<::Float> {
        self.0 * cgmath::Vector3::unit_z()
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction(cgmath::Quaternion::from_angle_y(cgmath::Deg(0.0)))
    }
}

impl specs::Component for Spatial {
    type Storage = specs::VecStorage<Self>;
}

impl specs::Component for Direction {
    type Storage = specs::VecStorage<Self>;
}
