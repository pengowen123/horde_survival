//! Systems and components related to graphics

#[macro_use]
extern crate gfx;
extern crate rendergraph;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate shred_derive;
extern crate common;
extern crate genmesh;
extern crate image_utils;
extern crate math;
extern crate obj;
extern crate ui;
extern crate window;
#[macro_use]
extern crate slog;
extern crate assets;

mod camera;
mod animation;
pub mod draw;
pub mod obj_loading;
pub mod particles;

use common::specs::{self, DispatcherBuilder};
use common::{cgmath, config, gfx_device_gl, gfx_window_glutin, glutin, shred, Float};

use std::sync::{Arc, Mutex};

/// Initializes graphics-related components and systems
pub fn initialize<'a, 'b, 'c, 'd>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
    dispatcher_graphics: DispatcherBuilder<'c, 'd>,
    init_test_entities: Box<Fn(&mut specs::World, &mut gfx_device_gl::Factory)>,
) -> (
    DispatcherBuilder<'a, 'b>,
    DispatcherBuilder<'c, 'd>,
    glutin::EventsLoop,
) {
    // The camera resource must exist before calling draw::initialize
    world.add_resource(Arc::new(Mutex::new(camera::Camera::new_default(1.0, 45.0))));

    // This must be initialized before init_test_entities is called
    let dispatcher = particles::initialize::<gfx_device_gl::Resources>(world, dispatcher);

    // Initialize subsystems
    let (dispatcher, dispatcher_graphics, window, events) =
        draw::initialize(world, dispatcher, dispatcher_graphics, init_test_entities);

    // Add resources
    {
        let window_size = {
            let log = world.read_resource::<slog::Logger>();
            window.get_window().get_inner_size().unwrap_or_else(|| {
                error!(log, "Failed to get window size (window probably doesn't exist anymore)";);
                panic!(common::CRASH_MSG);
            })
        };
        let camera = world.read_resource::<Arc<Mutex<camera::Camera>>>();
        let config = world.read_resource::<config::Config>();
        let aspect_ratio = (window_size.width / window_size.height) as f32;
        *camera.lock().unwrap() = camera::Camera::new_default(aspect_ratio, config.camera.fov);
    }
    world.add_resource(window);

    // Add systems
    let dispatcher = dispatcher
        // NOTE: The camera system can't depend on the window info system, so it will always be a
        //       frame behind. This should be fine because the window shouldn't be resized often.
        .with(camera::System, "camera", &[]);

    (dispatcher, dispatcher_graphics, events)
}
