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

// Logging
#[macro_use]
extern crate log;
extern crate log_panics;

// Misc
extern crate rand;
extern crate time;
extern crate random_choice;

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

fn main() {
    // Initialize logger
    hslog::init();
    log_panics::init();

    info!("Initializing game...");

    let mut game = gamestate::GameState::new();
    let options = hsgraphics::GraphicsOptions::new()
        .minimap_enabled(false)
        .display_debug(true)
        .crosshair(true)
        .clone();
    let (mut graphics, window) = hsgraphics::GraphicsState::new(options, &game);
    let mut loop_type = gameloop::LoopType::GUI;
    let mut ticks = tps::Ticks::new();
    let mut ui = gui::UI::new(&mut graphics);

    info!("Done");

    // Use this for testing the game
    game.entities[0].current_weapon = consts::items::weapon::TEST_GUN;
    game.entities[0].armor[0] = consts::items::armor::HEAL;

    // Event loop
    let mut events = window.poll_events();

    loop {
        let event = events.next();

        match loop_type {
            gameloop::LoopType::Game => gameloop::gametick(event, &mut ui, &mut game, &mut graphics, &window, &mut ticks, &mut loop_type),
            gameloop::LoopType::GUI => gameloop::run_gui(event, &mut ui, &mut game, &mut graphics, &window, &mut ticks, &mut loop_type),
        }

        if graphics.should_close {
            info!("Closed Horde Survival");
            break;
        }
    }
}
