#![feature(const_fn)]

// Graphics
#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate cgmath;
extern crate image;
extern crate collision;
extern crate rusttype;

// Logging
#[macro_use]
extern crate log;
extern crate log_panics;

// Misc
extern crate rand;
extern crate time;
extern crate random_choice;
extern crate image_utils;

#[macro_use]
mod hslog;
mod utils;
mod entity;
#[macro_use]
mod items;
mod world;
mod player;
mod consts;
mod hscontrols;
mod hsgraphics;
mod gamestate;
mod map;
mod minimap;
mod gameloop;
mod gui;
mod tps;
mod assets;

fn main() {
    // Initialize logger
    hslog::init();
    log_panics::init();

    info!("Initializing game...");

    // Initialize states
    let mut game = gamestate::GameState::new();
    let options = hsgraphics::GraphicsOptions::new()
        .window_size(1200, 900)
        .minimap_enabled(false)
        .display_debug(true)
        .crosshair(true)
        .fullscreen(false)
        .clone();

    let (mut graphics, window) = hsgraphics::GraphicsState::new(options, &game);
    let mut loop_type = gameloop::LoopType::Loading;
    let mut ticks = tps::Ticks::new();
    let mut ui = gui::UI::new();

    info!("Done");

    // Event loop
    let mut events = window.poll_events();

    loop {
        let event = events.next();

        match loop_type {
            gameloop::LoopType::Loading => gameloop::loading_screen(&mut ui, &mut graphics, &window, &mut loop_type),
            gameloop::LoopType::Game => gameloop::gametick(event, &mut ui, &mut game, &mut graphics, &window, &mut ticks, &mut loop_type),
            gameloop::LoopType::GUI => gameloop::run_gui(event, &mut ui, &mut game, &mut graphics, &window, &mut ticks, &mut loop_type),
        }

        if graphics.should_close {
            info!("Closed Horde Survival");
            break;
        }
    }
}
