//! Lazy initialization of physics bodies

use nphysics3d::object::{RigidBody, RigidBodyHandle};

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
