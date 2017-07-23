// TODO: Crate docs

// Entity component system
extern crate specs;
extern crate shred;
#[macro_use]
extern crate shred_derive;

// Physics
extern crate nphysics3d;
extern crate ncollide;

// Graphics
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

// Format-specific libraries
extern crate obj;

// Math
extern crate cgmath;
extern crate nalgebra as na;
extern crate alga;

// Logging
#[macro_use]
extern crate log;

// Misc
extern crate rayon;
extern crate time;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate quick_error;
extern crate genmesh;

// TODO: Remove when no longer needed
mod dev;

mod assets;
mod world;
mod physics;
mod math;
mod delta;
mod player;
mod control;
mod window;
mod graphics;

/// The floating point type used in this crate
pub type Float = f64;

// TODO: Docs
// TODO: Decide how systems should depend on each other (i think delta should come first always)
pub fn run() {
    // Create world
    let mut world = specs::World::new();
    let dispatcher = specs::DispatcherBuilder::new();

    // Call initialization function of each module (initializes their components and systems)
    let (dispatcher, sender) = player::init(&mut world, dispatcher);
    let dispatcher = control::init(&mut world, dispatcher);
    let dispatcher = delta::init(&mut world, dispatcher);
    let dispatcher = world::init(&mut world, dispatcher);
    // NOTE: This should be called before `graphics::init` so physics runs first
    let dispatcher = physics::init::init(&mut world, dispatcher);
    let (dispatcher, window, mut events) = graphics::init(&mut world, dispatcher);

    // Build the dispatcher
    let mut dispatcher = dispatcher.build();

    // Run systems
    loop {
        let mut latest_mouse_move = None;

        events.poll_events(|e| match e {
            glutin::Event::WindowEvent { event, .. } => {
                if let glutin::WindowEvent::MouseMoved { .. } = event {
                    latest_mouse_move = Some(event);
                    return;
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
        // NOTE: Running this after dispatch may be a problem (but so is running it before dispatch)
        world.maintain();
    }
}
