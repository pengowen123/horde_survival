//! Initialization of the physics system

use specs::{self, DispatcherBuilder};
use nphysics3d;
use na;
use common::physics;

use output;
use System;

/// Initializes physics-related systems and components
pub fn initialize<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {

    // Register components
    world.register::<physics::Physics>();

    // Add resources
    let mut physics_world = nphysics3d::world::World::new();
    physics_world.set_gravity(na::Vector3::new(0.0, 0.0, -9.81));

    world.add_resource(physics_world);

    // Initialize systems
    let system = System;

    // Add systems
    let dispatcher = dispatcher
        // This should depend on the delta system, but it can't because the delta system is run in a
        // separate dispatcher after the main run is run
        .with(system, "physics", &[]);

    // Initialize subsystems
    let dispatcher = output::initialize(dispatcher);

    dispatcher
}
