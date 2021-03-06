// TODO: Crate docs

#[macro_use]
extern crate shred_derive;
#[macro_use]
extern crate slog;
extern crate assets;
extern crate common;
extern crate control;
extern crate graphics;
extern crate image_utils;
extern crate math;
extern crate physics;
extern crate slog_async;
extern crate slog_term;
extern crate ui;
extern crate window;

// TODO: Remove when no longer needed
mod dev;

mod player;
mod player_control;

use common::shred;
use common::{config, glutin, specs, Float};
use window::window_event;

use std::sync::{mpsc, Arc};

// TODO: Docs
// TODO: Decide how systems should depend on each other (i think delta should come first always)
pub fn run(
    config: config::Config,
    cli_config: config::CommandLineConfig,
    logger: slog::Logger,
) -> config::Config {
    // Create world
    let mut world = specs::World::new();
    // Create a dispatcher for main systems (such as controls)
    let dispatcher = specs::DispatcherBuilder::new();
    // Create a dispatcher for graphics systems (such as graphics and ui)
    // This dispatcher is also used to run systems that must be run while not in-game (such as when
    // in a menu)
    let dispatcher_graphics = specs::DispatcherBuilder::new();

    // Add assets manager resource
    let assets = assets::Assets::new(&logger, cli_config.assets_path()).unwrap_or_else(|e| {
        error!(logger, "Error building asset manager: {}", e;);
        panic!(common::CRASH_MSG);
    });
    world.add_resource(Arc::new(assets));
    // Add logger resource
    world.add_resource(logger);
    // Add config resource
    world.add_resource(config);

    // Call initialization functions (initializes their components and systems)
    let dispatcher_graphics = common::initialize(&mut world, dispatcher_graphics);
    let dispatcher_graphics = window::initialize(&mut world, dispatcher_graphics);

    let dispatcher = player_control::initialize(&mut world, dispatcher);

    let dispatcher = control::initialize(&mut world, dispatcher);
    let dispatcher = physics::initialize(&mut world, dispatcher);
    ui::add_resources(&mut world);
    let (dispatcher, dispatcher_graphics, mut events) = graphics::initialize(
        &mut world,
        dispatcher,
        dispatcher_graphics,
        Box::new(dev::add_test_entities),
    );
    let physical_window_size = {
        let window = world.read_resource::<window::Window>();
        let window = window.get_window();
        window
            .get_inner_size()
            .unwrap()
            .to_physical(window.get_hidpi_factor())
            .into()
    };
    let (ui_event_sender, ui_event_receiver) = mpsc::channel();
    let dispatcher_graphics = ui::initialize(
        &mut world,
        dispatcher_graphics,
        physical_window_size,
        ui_event_receiver,
    );

    // Build the dispatchers
    let mut dispatcher = dispatcher.build();
    let mut dispatcher_graphics = dispatcher_graphics.build();

    // Run systems
    loop {
        let ui_state = *world.read_resource::<common::UiState>();
        if ui_state.should_exit() {
            break;
        }

        {
            let config = world.read_resource::<config::Config>();
            let window = world.read_resource::<window::Window>();
            let window = window.get_window();
            let mut channel = world.write_resource::<window_event::EventChannel>();

            let mut latest_mouse_move = None;

            events.poll_events(|e| {
                ui_event_sender
                    .send(e.clone())
                    .expect("Failed to send window event to UI system");

                match e {
                    glutin::Event::WindowEvent { event, .. } => {
                        let mut ui_state = world.write_resource::<common::UiState>();
                        let log = world.read_resource::<slog::Logger>();

                        window_event::process_window_event_graphics(
                            &mut channel,
                            &window,
                            &event,
                            &mut ui_state,
                            &log,
                        );

                        // If the game isn't running, only call process_window_event_graphics
                        if !ui_state.is_in_game() {
                            return;
                        }

                        // Collect the latest mouse event
                        if let glutin::WindowEvent::CursorMoved { .. } = event {
                            latest_mouse_move = Some(event);
                            return;
                        }

                        window_event::process_window_event(&config, &mut channel, &window, &event);
                    }
                    _ => {}
                }
            });

            // Only process the latest mouse movement event
            // NOTE: This won't be run if !ui_state.is_in_game() because of the above code
            if let Some(event) = latest_mouse_move {
                window_event::process_window_event(&config, &mut channel, &window, &event);
            }
        }

        // If the game is running (not in a menu), run main systems
        if ui_state.is_in_game() {
            dispatcher.dispatch(&mut world.res);
        }

        // Run graphics systems regardless of the UI state
        dispatcher_graphics.dispatch(&mut world.res);

        // NOTE: Running this after dispatch may be a problem (but so is running it before dispatch)
        world.maintain();
    }

    // Return the config so it can be written to the config file
    let config = world.read_resource::<config::Config>();
    config.clone()
}
