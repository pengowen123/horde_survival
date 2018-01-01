//! Systems and components related to graphics

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate rendergraph;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate shred_derive;
extern crate image_utils;
extern crate common;
extern crate window;
extern crate math;
extern crate genmesh;
extern crate regex;
extern crate obj;
#[macro_use]
extern crate lazy_static;

// TODO: Remove when `dev` module is gone
extern crate gfx_device_gl;

pub mod draw;
pub mod assets;
mod camera;

use common::{Float, shred, glutin};
use common::specs::{self, DispatcherBuilder};

/// Initializes graphics-related components and systems
pub fn initialize<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
    init_test_entities: Box<Fn(&mut specs::World, &mut gfx_device_gl::Factory)>,
) -> (DispatcherBuilder<'a, 'b>, window::Window, glutin::EventsLoop) {
    // Initialize subsystems
    let (dispatcher, window, events) = draw::initialize(world, dispatcher, init_test_entities);

    // Add resources
    let (w, h) = window.get_inner_size().unwrap();
    world.add_resource(camera::Camera::new_default(w as f32 / h as f32));
    world.add_resource(window.clone());

    // Add systems
    let dispatcher = dispatcher
        .add(camera::System, "camera", &["window-info"]);

    (dispatcher, window, events)
}
