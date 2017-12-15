// TODO: Crate docs

// Entity component system
use common::specs;
extern crate shred;
#[macro_use]
extern crate shred_derive;

// Physics
extern crate physics;

// Graphics
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
use common::glutin;
extern crate window;

// Assets
extern crate obj;
extern crate image_utils;

// Math
use common::{na, cgmath};
extern crate math;

// Misc
extern crate rayon;
#[macro_use]
extern crate quick_error;
extern crate genmesh;
extern crate regex;
#[macro_use]
extern crate lazy_static;

// Common types
extern crate common;

// Dev dependencies
#[cfg(test)]
#[macro_use]
extern crate approx;

// TODO: Remove when no longer needed
mod dev;

mod assets;
mod player;
mod control;
mod graphics;

use shred::RunNow;

/// The floating point type used in this crate
use common::Float;

// TODO: Docs
// TODO: Decide how systems should depend on each other (i think delta should come first always)
pub fn run() {
    // Create world
    let mut world = specs::World::new();
    let dispatcher = specs::DispatcherBuilder::new();

    // Call initialization functions (initializes their components and systems)
    let dispatcher = common::initialize(&mut world, dispatcher);
    let dispatcher = window::initialize(&mut world, dispatcher);
    let (dispatcher, sender) = player::init(&mut world, dispatcher);
    let dispatcher = control::init(&mut world, dispatcher);
    let (dispatcher, mut physics) = physics::initialize(&mut world, dispatcher);
    let (dispatcher, window, mut events) = graphics::init(&mut world, dispatcher);

    // Build the dispatcher
    let mut dispatcher = dispatcher.build();

    let mut running = true;

    // Run systems
    while running {
        let mut latest_mouse_move = None;

        events.poll_events(|e| match e {
            glutin::Event::WindowEvent { event, .. } => {
                // Collect the latest mouse event
                if let glutin::WindowEvent::MouseMoved { .. } = event {
                    latest_mouse_move = Some(event);
                    return;
                // Test if `Escape` was pressed, and if so, end the event loop
                } else if let glutin::WindowEvent::KeyboardInput { input, .. } = event {
                    if let Some(glutin::VirtualKeyCode::Escape) = input.virtual_keycode {
                        if let glutin::ElementState::Pressed = input.state {
                            running = false;
                        }
                    }
                }

                sender.process_window_event(&window, event);
            }
            _ => {}
        });

        // Only process the latest mouse movement event
        if let Some(event) = latest_mouse_move {
            sender.process_window_event(&window, event)
        }

        dispatcher.dispatch(&mut world.res);

        // nphysics world is not threadsafe so the system is run manually
        physics.run_now(&mut world.res);

        // NOTE: Running this after dispatch may be a problem (but so is running it before dispatch)
        world.maintain();
    }
}
