//! Systems and components to abstract the use of the game window and its events

extern crate common;
#[macro_use]
extern crate shred_derive;
use common::{shred, specs};
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate slog;

pub mod info;
pub mod config;
pub mod window_event;
pub mod input;

use common::{Float, glutin};

use std::sync::Arc;

pub type Window = Arc<glutin::GlWindow>;

/// Registers all components and systems in this crate
pub fn initialize<'a, 'b>(
    world: &mut specs::World,
    dispatcher: specs::DispatcherBuilder<'a, 'b>,
) -> specs::DispatcherBuilder<'a, 'b> {

    world.add_resource(info::WindowInfo::default());
    world.add_resource(window_event::EventChannel::new());

    let mut event_channel = world.write_resource::<window_event::EventChannel>();
    let reader_id = event_channel.register_reader();

    let config_system = config::System::new(reader_id);
    // NOTE: These systems will be added to the graphics dispatcher, if other systems are added here
    //       in the future the main dispatcher must be added as an argument to this function
    dispatcher
        .with(info::System, "window-info", &[])
        .with(config_system, "window-config", &[])
}
