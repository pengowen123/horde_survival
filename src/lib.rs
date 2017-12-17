// TODO: Crate docs

#[macro_use]
extern crate shred_derive;
extern crate common;
extern crate math;
extern crate physics;
extern crate window;
extern crate control;
extern crate graphics;

// TODO: Remove when no longer needed
mod dev;

mod player_control;

use common::{specs, glutin};
use common::shred::{self, RunNow};

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
    let (dispatcher, sender) = player_control::initialize(dispatcher);
    let dispatcher = control::initialize(&mut world, dispatcher);
    let (dispatcher, mut physics) = physics::initialize(&mut world, dispatcher);
    let (dispatcher, window, mut events) = graphics::initialize(&mut world, dispatcher,
                                                                Box::new(dev::add_test_entities));

    // Build the dispatcher
    let mut dispatcher = dispatcher.build();

    let mut running = true;

    // Run systems
    while running {
        let mut latest_mouse_move = None;

        events.poll_events(|e| match e {
            glutin::Event::WindowEvent { event, .. } => {
                // Collect the latest mouse event
                if let glutin::WindowEvent::CursorMoved { .. } = event {
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
