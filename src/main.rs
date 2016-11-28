#![cfg_attr(not(feature="clippy"), allow(unknown_lints))]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

// Graphics
#[macro_use]
pub extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate cgmath;
extern crate image;
#[macro_use]
extern crate conrod;

// Logging
#[macro_use]
extern crate log;
extern crate log_panics;

// Misc
extern crate rand;
extern crate time;
extern crate random_choice;
extern crate image_utils;
extern crate collision;
extern crate shader_version;

#[macro_use]
mod utils;
#[macro_use]
mod hslog;
#[macro_use]
mod entity;
#[macro_use]
mod items;
#[macro_use]
mod hsgraphics;
#[macro_use]
mod world;
mod player;
mod consts;
mod hscontrols;
mod gamestate;
mod map;
mod gameloop;
mod gui;
mod tps;
mod assets;
mod platform;

fn main() {
    // Initialize logger
    hslog::init();
    log_panics::init();

    info!("Initializing game...");

    // Initialize states
    let mut game = gamestate::GameState::new();
    let options = hsgraphics::GraphicsOptions::new()
        .window_size(800, 600)
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
