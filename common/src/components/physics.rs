/// Physics components

use specs;
use nphysics3d::object::{RigidBody, RigidBodyHandle};

/// A component for any entity that should be simulated by the physics engine
pub struct Physics {
    handle: Handle,
    /// If true, lock the entity's orientation
    lock_rotation: bool,
}

impl Physics {
    /// Returns a new `Physics`, using `body_init` to create the physics body.
    /// If `lock_orientation` is true, the body will have its orientation locked
    pub fn new(body_init: BodyInit, lock_rotation: bool) -> Self {
        Physics {
            handle: Handle::Init(Some(body_init)),
            lock_rotation,
        }
    }

    /// Returns a reference to the handle to this entity's physics body
    pub fn handle(&self) -> &Handle {
        &self.handle
    }

    /// Returns a mutable reference to the handle to this entity's physics body
    pub fn handle_mut(&mut self) -> &mut Handle {
        &mut self.handle
    }

    /// Sets the physics body handle of this entity to the provided value
    pub fn set_handle(&mut self, handle: RigidBodyHandle<::Float>) {
        self.handle = Handle::Body(handle);
    }

    /// Returns whether to lock the orientation of this entity's physics body
    pub fn lock_rotation(&self) -> bool {
        self.lock_rotation
    }
}

/// This component acts as a flag to enable the overwriting of an entity's direction with the
/// direction of its physics body
#[derive(Default)]
pub struct PhysicsTiedDirection;

/// This component acts as a flag to enable the overwriting of an entity's position with the
/// position of its physics body
#[derive(Default)]
pub struct PhysicsTiedPosition;

/// A function that creates and returns physics body
pub type BodyInit = Box<FnMut() -> RigidBody<::Float> + Send + Sync + 'static>;

/// Either a handle to a physics body, or a function to initialize one
pub enum Handle {
    Body(RigidBodyHandle<::Float>),
    Init(Option<BodyInit>),
}

impl Handle {
    /// Returns the a mutable reference to the physics body if `self` is `Handle::Body`, otherwise
    /// returns `None`
    pub fn get_body_mut(&mut self) -> Option<&mut RigidBodyHandle<::Float>> {
        if let Handle::Body(ref mut rb) = *self {
            Some(rb)
        } else {
            None
        }
    }

    /// Calls the provided function on the handle if it exists
    pub fn map<F>(&self, f: F)
    where
        F: FnOnce(&RigidBodyHandle<::Float>),
    {
        match *self {
            Handle::Body(ref h) => f(h),
            Handle::Init(_) => {}
        }
    }
}

impl specs::Component for Physics {
    type Storage = specs::VecStorage<Self>;
}

impl specs::Component for PhysicsTiedPosition {
    type Storage = specs::NullStorage<Self>;
}

impl specs::Component for PhysicsTiedDirection {
    type Storage = specs::NullStorage<Self>;
}
