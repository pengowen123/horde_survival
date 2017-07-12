//! Components and systems related to the world

pub mod components;

use specs::{self, DispatcherBuilder};

/// Initialization of world-related components and systems
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {

    // Register components
    world.register::<components::Spatial>();
    world.register::<components::Direction>();
    world.register::<components::Control>();

    dispatcher
}
