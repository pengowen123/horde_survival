//! Lazy initialization of physics bodies

use nphysics3d::object::{RigidBody, RigidBodyHandle};

/// A function that creates and returns physics body
pub type BodyInit = Box<Fn() -> RigidBody<::Float> + Send + Sync + 'static>;

/// Either a handle to a physics body, or a function to initialize one
pub enum Handle {
    Body(RigidBodyHandle<::Float>),
    Init(BodyInit),
}

impl Handle {
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
