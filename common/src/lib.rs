//! Common dependencies and types shared between crates

pub extern crate specs;
pub extern crate shred;
pub extern crate cgmath;
pub extern crate nalgebra as na;
pub extern crate ncollide;
pub extern crate nphysics3d;
pub extern crate time;
#[macro_use]
pub extern crate log;

mod delta;
mod components;

pub use self::components::*;
pub use self::delta::*;

pub type Float = f64;

/// Registers all components and systems in this crate
pub fn initialize<'a, 'b>(
    world: &mut specs::World,
    dispatcher: specs::DispatcherBuilder<'a, 'b>,
) -> specs::DispatcherBuilder<'a, 'b> {

    world.register::<components::Position>();
    world.register::<components::Direction>();
    world.register::<components::Scale>();

    world.add_resource(Delta::default());

    dispatcher.add(System::new(), "delta-time", &[])
}
