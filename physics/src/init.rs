//! Initialization of the physics system

use specs::{self, DispatcherBuilder};
use nphysics3d;
use na;
use common::physics;

use output;
use System;

/// Initializes physics-related components, and returns a new physics `System`
///
/// The system cannot be added to the dispatcher and instead must be run with `system.run_now()`
pub fn initialize<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> (DispatcherBuilder<'a, 'b>, System) {

    // Register components
    world.register::<physics::Physics>();

    // Initialize systems
    let mut physics_world = nphysics3d::world::World::new();
    physics_world.set_gravity(na::Vector3::new(0.0, 0.0, -9.81));

    let system = System { world: physics_world };

    // Initialize subsystems
    let dispatcher = output::initialize(dispatcher);

    (dispatcher, system)
}
