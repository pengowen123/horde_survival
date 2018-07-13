//! Systems and components related to graphics

#[macro_use]
extern crate gfx;
extern crate rendergraph;
#[macro_use]
extern crate quick_error;
#[macro_use]
extern crate shred_derive;
extern crate image_utils;
extern crate common;
extern crate window;
extern crate math;
extern crate ui;
extern crate genmesh;
extern crate regex;
extern crate obj;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;

pub mod draw;
pub mod assets;
mod camera;

use common::{Float, cgmath, shred, glutin, gfx_window_glutin, gfx_device_gl};
use common::specs::{self, DispatcherBuilder};

use std::sync::{Arc, Mutex};

/// Initializes graphics-related components and systems
pub fn initialize<'a, 'b, 'c, 'd>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
    dispatcher_graphics: DispatcherBuilder<'c, 'd>,
    init_test_entities: Box<Fn(&mut specs::World, &mut gfx_device_gl::Factory)>,
) -> (DispatcherBuilder<'a, 'b>,
      DispatcherBuilder<'c, 'd>,
      window::Window,
      glutin::EventsLoop)
{
    // The camera resource must exist before calling draw::initialize
    world.add_resource(Arc::new(Mutex::new(camera::Camera::new_default(1.0))));

    // Initialize subsystems
    let (dispatcher, dispatcher_graphics, window, events) =
        draw::initialize(world, dispatcher, dispatcher_graphics, init_test_entities);

    // Add resources
    {
        let window_size = {
            let log = world.read_resource::<slog::Logger>();
            window.get_inner_size().unwrap_or_else(|| {
                error!(log, "Failed to get window size (window probably doesn't exist anymore)";);
                panic!(common::CRASH_MSG);
            })
        };
        let camera = world.write_resource::<Arc<Mutex<camera::Camera>>>();
        *camera.lock().unwrap() = camera::Camera::new_default(window_size.width as f32 /
                                                              window_size.height as f32);
    }
    world.add_resource(window.clone());

    // Add systems
    let dispatcher = dispatcher
        // NOTE: The camera system can't depend on the window info system, so it will always be a
        //       frame behind. This should be fine because the window shouldn't be resized often.
        .add(camera::System, "camera", &[]);

    (dispatcher, dispatcher_graphics, window, events)
}
