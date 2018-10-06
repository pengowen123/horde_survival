//! Common components

pub mod graphics;
pub mod physics;

use cgmath::{self, InnerSpace, Rotation3};
use specs;

/// A flag that represents an entity being a player entity
///
/// Entities with this component will be controlled by the player.
///
/// This component must only exist on one entity (failure to uphold this may cause unexpected
/// behavior)
#[derive(Clone, Copy, Debug, Default)]
pub struct Player;

/// The position of an entity
#[derive(Clone, Copy, Debug)]
pub struct Position(pub cgmath::Point3<::Float>);

/// The direction of an entity
#[derive(Clone, Copy, Debug)]
pub struct Direction(pub cgmath::Quaternion<::Float>);

/// The scale of an entity
// The second field indicates whether to update the physics body's scale
#[derive(Clone, Copy)]
pub struct Scale(f32, Option<f32>);

impl Scale {
    pub fn new(val: f32) -> Self {
        // The previous value is set to Some(..) so the physics body will be updated
        Scale(val, Some(1.0))
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    /// Sets the scale to the provided value
    ///
    /// If `update_physics_body` is true, the scale of this entity's physics body will be adjusted
    /// to the new scale
    pub fn set(&mut self, val: f32, update_physics_body: bool) {
        if update_physics_body {
            self.1 = Some(self.0);
        }

        self.0 = val;
    }

    /// Returns the previously stored value if it exists
    pub fn get_previous_value(&self) -> Option<f32> {
        self.1
    }

    /// Resets the flag that causes the physics system to update the scale of this entity's body
    pub fn reset_flag(&mut self) {
        self.1 = None;
    }
}

impl Default for Scale {
    fn default() -> Self {
        Scale(1.0, None)
    }
}

impl From<Direction> for cgmath::Vector3<::Float> {
    /// Converts this `Direction` to a direction vector
    fn from(val: Direction) -> cgmath::Vector3<::Float> {
        val.0 * cgmath::Vector3::unit_z()
    }
}

impl From<cgmath::Vector3<::Float>> for Direction {
    /// Converts the direction vector into a `Direction`
    fn from(val: cgmath::Vector3<::Float>) -> Self {
        let val = val.normalize();

        let quat = cgmath::Quaternion::from_arc(cgmath::Vector3::unit_z(), val, None);

        Direction(quat)
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction(cgmath::Quaternion::from_angle_y(cgmath::Deg(0.0)))
    }
}

impl specs::Component for Player {
    type Storage = specs::NullStorage<Self>;
}

impl specs::Component for Position {
    type Storage = specs::DenseVecStorage<Self>;
}

impl specs::Component for Direction {
    type Storage = specs::DenseVecStorage<Self>;
}

impl specs::Component for Scale {
    type Storage = specs::DenseVecStorage<Self>;
}
