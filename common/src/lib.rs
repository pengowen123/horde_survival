//! Common dependencies and types shared between crates

pub extern crate specs;
pub extern crate cgmath;

mod components;

pub use self::components::*;

pub type Float = f64;

/// Registers all components in this crate
pub fn register_components(world: &mut specs::World) {
    world.register::<components::Position>();
    world.register::<components::Direction>();
}
