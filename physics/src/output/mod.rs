//! Systems to tie properties of an entity's physics body to other components

pub mod direction;
pub mod position;

use common::na;
use nphysics3d::object::{BodyHandle, RigidBody, Multibody, Ground, BodyPart};
use nphysics3d::world::World;
use specs::DispatcherBuilder;

pub fn initialize<'a, 'b>(dispatcher: DispatcherBuilder<'a, 'b>) -> DispatcherBuilder<'a, 'b> {
    // Add systems
    dispatcher
        .with(position::System, "physics-tied-position", &["physics"])
        .with(direction::System, "physics-tied-direction", &["physics"])
}

/// Returns the isometry of the collider with the provided handle
///
/// For multibodies, the isometry of the first link is returned.
pub fn get_isometry(world: &World<::Float>, handle: BodyHandle) -> na::Isometry3<::Float> {
    if let Some(body) = world.body(handle) {
        if let Some(rb) = body.downcast_ref::<RigidBody<::Float>>() {
            return rb.position().clone();
        } else if let Some(ground) = body.downcast_ref::<Ground<::Float>>() {
            return ground.position();
        } else if let Some(multibody) = body.downcast_ref::<Multibody<::Float>>() {
            return multibody.root().position();
        }
    }

    // All possible body types are handled above
    unreachable!();
}
