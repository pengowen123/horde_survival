//! Common dependencies and types shared between crates

// ECS
pub extern crate specs;
pub extern crate shred;

// Math
pub extern crate cgmath;
pub extern crate nalgebra as na;

// Physics
pub extern crate ncollide;
pub extern crate nphysics3d;

// Graphics
pub extern crate glutin;
pub extern crate gfx;
pub extern crate gfx_window_glutin;
pub extern crate gfx_device_gl;

// Misc
pub extern crate time;
#[macro_use]
pub extern crate log;

mod delta;
mod components;

pub use self::components::*;
pub use self::delta::*;

/// The float type used in `horde_survival`
pub type Float = f64;

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

    dispatcher.add(System::new(), "delta-time", &[])
}
