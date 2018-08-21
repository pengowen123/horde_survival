/// Physics components

use specs;
use nphysics3d::object::{ColliderHandle, BodyHandle};

/// A component for any entity that should be simulated by the physics engine
pub struct Physics {
    /// The handle of the root body of this entity
    root_handle: BodyHandle,
    /// A list of handles to the child bodies of the root body of this entity
    child_handles: Vec<BodyHandle>,
    /// The handle of the root collider of this entity
    root_collider: Option<ColliderHandle>,
    /// A list of handles to the child colliders of the root collider of this entity
    collider_handles: Vec<ColliderHandle>,
}

impl Physics {
    pub fn new(
        root_handle: BodyHandle,
        child_handles: Vec<BodyHandle>,
        root_collider: Option<ColliderHandle>,
        collider_handles: Vec<ColliderHandle>,
    ) -> Self {
        Physics {
            root_handle,
            child_handles,
            root_collider,
            collider_handles,
        }
    }

    /// Returns the handle to this entity's root physics body
    pub fn get_root_handle(&self) -> BodyHandle {
        self.root_handle
    }

    /// Returns a reference to the handles to this entity's child physics bodies
    pub fn get_child_handles(&self) -> &[BodyHandle] {
        &self.child_handles
    }

    /// Returns the handle to this entity's root collider
    pub fn get_root_collider(&self) -> Option<ColliderHandle> {
        self.root_collider
    }

    /// Returns a reference to the handles to this entity's child colliders
    pub fn get_child_colliders(&self) -> &[ColliderHandle] {
        &self.collider_handles
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

impl specs::Component for Physics {
    type Storage = specs::VecStorage<Self>;
}

impl specs::Component for PhysicsTiedPosition {
    type Storage = specs::NullStorage<Self>;
}

impl specs::Component for PhysicsTiedDirection {
    type Storage = specs::NullStorage<Self>;
}
