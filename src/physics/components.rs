//! Components related to the physics system

use specs;
use nphysics3d::object::RigidBodyHandle;

use super::handle;

pub use super::output::position::PhysicsTiedPosition;
pub use super::output::direction::PhysicsTiedDirection;

/// A component for any entity that should be simulated by the physics engine
pub struct Physics {
    handle: handle::Handle,
    /// If true, lock the entity's orientation
    lock_rotation: bool,
}

impl Physics {
    /// Returns a new `Physics`, using `body_init` to create the physics body.
    /// If `lock_orientation` is true, the body will have its orientation locked
    pub fn new(body_init: handle::BodyInit, lock_rotation: bool) -> Self {
        Physics {
            handle: handle::Handle::Init(Some(body_init)),
            lock_rotation,
        }
    }

    /// Returns a reference to the handle to this entity's physics body
    pub fn handle(&self) -> &handle::Handle {
        &self.handle
    }

    /// Returns a mutable reference to the handle to this entity's physics body
    pub fn handle_mut(&mut self) -> &mut handle::Handle {
        &mut self.handle
    }

    /// Sets the physics body handle of this entity to the provided value
    pub fn set_handle(&mut self, handle: RigidBodyHandle<::Float>) {
        self.handle = handle::Handle::Body(handle);
    }

    /// Returns whether to lock the orientation of this entity's physics body
    pub fn lock_rotation(&self) -> bool {
        self.lock_rotation
    }
}

impl specs::Component for Physics {
    type Storage = specs::VecStorage<Self>;
}
