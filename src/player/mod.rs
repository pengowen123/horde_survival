//! Components and systems related to the player

pub mod control;

use specs::{self, DispatcherBuilder};

use window::window_event;

/// Initializes player-related components
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> (DispatcherBuilder<'a, 'b>, window_event::SenderHub) {

    // Initialize systems
    let (snd, recv) = window_event::SenderHub::new();
    let control = control::System::new(recv.into_receiver());

    // Add systems
    let dispatcher = dispatcher.add(control, "player-control", &[]);

    (dispatcher, snd)
}
