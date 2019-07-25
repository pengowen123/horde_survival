//! Components and systems for physics simulation
//!
//! The physics system internally uses the `nphysics3d` physics engine, so it exists as a wrapper
//! with components to provide input to the engine, and to read from the results.

extern crate common;
extern crate math;
#[macro_use]
extern crate shred_derive;

mod init;
mod output;
pub mod scale;

pub use init::initialize;

#[allow(unused_imports)]
use common::shred::{self, SystemData, ResourceId, Resources};
use common::{specs, physics, na, ncollide3d, nphysics3d, Delta, Float};

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    world: specs::WriteExpect<'a, nphysics3d::world::World<::Float>>,
    physics: specs::ReadStorage<'a, physics::Physics>,
    delta: specs::ReadExpect<'a, Delta>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        let delta = data.delta.to_float();

        self.remove_dead_bodies();

        // Simulate the world for `delta` seconds
        data.world.set_timestep(delta);
        data.world.step();
    }
}

impl System {
    /// Removes physics bodies belonging to entities that were removed
    // TODO: Implement this
    fn remove_dead_bodies(&mut self) {}
}
