//! Systems to tie properties of an entity's physics body to other components

pub mod position;
pub mod direction;

use specs::DispatcherBuilder;

pub fn initialize<'a, 'b>(
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> DispatcherBuilder<'a, 'b> {
    // Add systems
    // NOTE: Until the physics system is not thread-local, these will be reading slightly outdated
    //       values (shouldn't be a big deal though)
    dispatcher
        .with(position::System, "physics-tied-position", &[])
        .with(direction::System, "physics-tied-direction", &[])
}
