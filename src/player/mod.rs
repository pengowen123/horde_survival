//! Components and systems related to the player

pub mod components;
pub mod control;

use specs::{self, DispatcherBuilder};

use window::event;

/// Initializes player-related components
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> (DispatcherBuilder<'a, 'b>, event::SenderHub) {

    // Register components
    world.register::<components::Player>();

    // Initialize systems

    let (snd, recv) = event::SenderHub::new();
    let control = control::System::new(recv.into_receiver());

    // Add systems
    let dispatcher = dispatcher.add(control, "player-control", &[]);

    (dispatcher, snd)
}
