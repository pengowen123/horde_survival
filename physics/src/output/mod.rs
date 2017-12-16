//! Systems to tie properties of an entity's physics body to other components

pub mod position;
pub mod direction;

use specs::{self, DispatcherBuilder};

pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    // Add systems
    // NOTE: Until the physics system is not thread-local, these will be reading slightly outdated
    //       values (shouldn't be a big deal though)
    dispatcher
        .add(position::System, "physics-tied-position", &[])
        .add(direction::System, "physics-tied-direction", &[])
}
