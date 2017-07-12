//! Initialization of the physics system

use specs::{self, DispatcherBuilder};
use nphysics3d;
use na;

use physics::{System, components, output};

/// Initializes physics-related components and systems
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {

    // Register components
    world.register::<components::Physics>();

    // Initialize systems
    let mut physics_world = nphysics3d::world::World::new();
    physics_world.set_gravity(na::Vector3::new(0.0, 0.0, -9.81));

    let system = System { world: physics_world };

    // Add systems
    // TODO: Make this parallel, look into the recursion error
    let dispatcher = dispatcher.add_thread_local(system);

    // Initialize subsystems
    output::init(world, dispatcher)
}
