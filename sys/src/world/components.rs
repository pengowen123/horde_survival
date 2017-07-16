//! Components related to the world

use specs;
use cgmath;

pub type Position = cgmath::Point3<::Float>;
pub type Velocity = cgmath::Vector3<::Float>;

#[derive(Clone, Copy, Debug)]
pub struct Spatial(pub Position);

#[derive(Clone, Copy, Debug)]
pub struct Direction(pub cgmath::Quaternion<::Float>);

/// Controlled movement properties, not necessarily by a player
pub struct Control {
    pub movement: Velocity,
}

impl specs::Component for Spatial {
    type Storage = specs::VecStorage<Self>;
}

impl specs::Component for Direction {
    type Storage = specs::VecStorage<Self>;
}

impl specs::Component for Control {
    type Storage = specs::VecStorage<Self>;
}
