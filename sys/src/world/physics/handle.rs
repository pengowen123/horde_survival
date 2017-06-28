//! Handles to bodies in the physics engine

use nphysics3d::object::{RigidBody, RigidBodyHandle};

use std::rc::Rc;

pub enum BodyHandle {
    Id(BodyId),
    New(BodyInit),
}

// NOTE: A function must be used rather than the object itself because nphysics rigid body handles
//      are not thread-safe
// NOTE: Think about making this a trait object
pub type BodyInit = fn() -> RigidBody<::Float>;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BodyId {
    id: usize,
}

impl BodyId {
    /// Returns a new `BodyId` and the body it identifies
    pub fn new(body: RigidBodyHandle<::Float>) -> (Self, RigidBodyHandle<::Float>) {
        let ptr = Rc::into_raw(body);
        let id = Self { id: ptr as usize };

        let body = unsafe { Rc::from_raw(ptr) };

        (id, body)
    }
}
