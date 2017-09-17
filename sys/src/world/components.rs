//! Components related to the world

use specs;
use cgmath;

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

impl specs::Component for Spatial {
    type Storage = specs::VecStorage<Self>;
}

impl specs::Component for Direction {
    type Storage = specs::VecStorage<Self>;
}
