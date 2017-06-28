//! Components related to the world

use specs;
use cgmath;

use std::ops::Deref;

use math::direction;
use world::physics::handle;

pub type Position = cgmath::Point3<::Float>;
pub type Velocity = cgmath::Vector3<::Float>;


/// A component for any entity that should be simulated by the physics engine
pub struct Physics {
    /// The ID of the handle to the entity's body in the physics engine
    /// Is `BodyId::` if the body has not been created yet (but will be next run of the physics system)
    //NOTE: This is necessary because nphysics rigid body handles are not thread-safe
    pub handle: handle::BodyHandle,
    /// If true, lock the entity's orientation
    lock_rotation: bool,
}

impl Physics {
    /// Returns a new `Physics`, using `body_init` to create the physics body.
    /// If `lock_orientation` is true, the body will have its orientation locked
    pub fn new(body_init: handle::BodyInit, lock_rotation: bool) -> Self {
        Physics {
            handle: handle::BodyHandle::New(body_init),
            lock_rotation,
        }
    }

    /// Returns whether to lock the orientation of this entity's body
    pub fn lock_rotation(&self) -> bool {
        self.lock_rotation
    }
}

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

impl specs::Component for Physics {
    type Storage = specs::VecStorage<Self>;
}
