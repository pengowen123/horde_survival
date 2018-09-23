//! Common dependencies and types shared between crates

// ECS
pub extern crate shred;
pub extern crate shrev;
pub extern crate specs;

// Math
pub extern crate cgmath;
pub extern crate nalgebra as na;

// Physics
pub extern crate ncollide3d;
pub extern crate nphysics3d;

// Graphics
pub extern crate gfx;
pub extern crate gfx_core;
pub extern crate gfx_device_gl;
pub extern crate gfx_window_glutin;
pub extern crate glutin;

// UI
pub extern crate conrod;

// Misc
pub extern crate time;
#[macro_use]
pub extern crate structopt;
extern crate serde;
extern crate slog;
#[macro_use]
extern crate serde_derive;

mod components;
pub mod config;
mod delta;
mod resources;
pub mod utils;

pub use self::components::*;
pub use self::delta::*;
pub use self::resources::*;

/// The float type used in `horde_survival`
pub type Float = f64;

/// The message that gets printed when Horde Survival crashes
pub const CRASH_MSG: &str = "An error has occurred";

/// Registers all components and systems in this crate
pub fn initialize<'a, 'b>(
    world: &mut specs::World,
    dispatcher: specs::DispatcherBuilder<'a, 'b>,
) -> specs::DispatcherBuilder<'a, 'b> {
    world.register::<components::Player>();
    world.register::<components::Position>();
    world.register::<components::Direction>();
    world.register::<components::Scale>();
    world.register::<components::physics::Physics>();
    world.register::<components::physics::PhysicsTiedPosition>();
    world.register::<components::physics::PhysicsTiedDirection>();

    world.add_resource(Delta::default());

    // NOTE: This system will be added to the graphics dispatcher, if other systems are added here
    //       in the future the main dispatcher must be added as an argument to this function
    dispatcher.with(System::new(), "delta-time", &[])
}
