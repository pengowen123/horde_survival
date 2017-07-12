//! Components related to the world

use specs;
use cgmath;

use std::ops::{Deref, DerefMut};

use math::direction;

pub type Position = cgmath::Point3<::Float>;
pub type Velocity = cgmath::Vector3<::Float>;

/// This component gets updated by the physics system, so it only exists as an input to the
/// rendering system
#[derive(Clone, Copy, Debug)]
pub struct Spatial {
    pub position: Position,
}

impl Spatial {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Direction(pub direction::Direction);

impl Deref for Direction {
    type Target = direction::Direction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Direction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
