//! Systems to tie properties of an entity's physics body to other components

pub mod position;
pub mod direction;

use specs::DispatcherBuilder;
use common::na;
use nphysics3d::world::World;
use nphysics3d::object::{Body, BodyHandle, Multibody, MultibodyLinkRef, MultibodyLinkMut};

pub fn initialize<'a, 'b>(
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    // Add systems
    dispatcher
        .with(position::System, "physics-tied-position", &["physics"])
        .with(direction::System, "physics-tied-direction", &["physics"])
}

/// Returns the isometry of the collider with the provided handle
///
/// For multibodies, the isometry of the first link is returned.
pub fn get_isometry(world: &World<::Float>, handle: BodyHandle) -> na::Isometry3<::Float> {
    match world.body(handle) {
        Body::RigidBody(b) => b.position(),
        Body::Ground(b) => b.position(),
        Body::Multibody(b) => {
            get_first_multibody_link(b).position()
        },
    }
}

/// Returns the first link in the provided multibody
pub fn get_first_multibody_link(multibody: &Multibody<::Float>) -> MultibodyLinkRef<::Float> {
    multibody
        .links()
        .next()
        .expect("`get_first_multibody_link` called with empty multibody")
}
