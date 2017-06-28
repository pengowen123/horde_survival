//! Components and systems related to the world

mod components;
mod physics;

pub use self::components::*;

use specs::{self, DispatcherBuilder};

/// Initialization of world-related components and systems
pub fn init<'a, 'b>(world: &mut specs::World,
                    dispatcher: DispatcherBuilder<'a, 'b>)
                    -> DispatcherBuilder<'a, 'b> {

    // Register components
    world.register::<Spatial>();
    world.register::<Direction>();
    world.register::<Control>();

    // Call submodule initialization functions
    let dispatcher = physics::init(world, dispatcher);

    dispatcher
}
