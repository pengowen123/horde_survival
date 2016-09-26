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

use utils::*;
use entity::*;
use consts::*;
use hsgraphics::*;
use gamestate::GameState;

use std::time::{Instant, Duration};

fn main() {
    // Initialize logger
    hslog::init();
    log_panics::init();

    info!("Initializing game...");

    let options = GraphicsOptions::new()
        .minimap_enabled(false)
        .display_debug(true)
        .crosshair(true)
        .clone();
    let (mut graphics, window) = GraphicsState::new(options);
    let mut game = GameState::new();

    info!("Done");

    // Use this for testing the game
    game.entities[0].current_weapon = TEST_GUN;
    game.entities[0].armor[0] = consts::items::armor::HEAL;

    // Used for keeping TPS below a certain value
    let mut time;
    let mut sleeping_until = Instant::now();
    let expected_elapsed = Duration::from_millis(1_000_000_000 / TPS / 1_000_000);

    // Event loop
    let mut events = window.poll_events();

    'main: loop {
        time = Instant::now();
        let event = events.next();

        if let Some(e) = event {
            if gameloop::handle_event(e, &mut game, &mut graphics, &window) {
                info!("Closed Horde Survival");
                break 'main;
            }
        }

        graphics.draw(&window);

        let t2 = Instant::now();
        let t_gfx = t2 - time;

        // Skip game updates until the tick has finished
        if Instant::now() < sleeping_until { continue; }

        gameloop::update_player_state(&mut game, &mut graphics, &window);

        for i in 0..game.entities.len() {
            update_entity(&mut game.entities, i, &game.map, &mut game.player, &mut game.next_entity_id);
        }

        //update_clumped_entities(&mut game.entities);
        filter_entities(&mut game.entities);

        if game.round_finished() {
            game.next_round();
        }

        let t3 = Instant::now();
        let t_game = t3 - t2;

        graphics.update(&game);

        let t_gfx_2 = Instant::now() - t3;

        // Set duration to skip game loop for
        let current = Instant::now();
        let elapsed = current - time;

        if graphics.options.display_debug {
            let s1 = millis(t_gfx + t_gfx_2);
            let s2 = millis(t_game);

            let string = format!("Horde Survival - frame {} ms | updates {} ms | total {} ms", s1, s2, millis(elapsed));

            window.set_title(&string);
        }

        if elapsed < expected_elapsed {
            sleeping_until = current + (expected_elapsed - elapsed);
        }
    }
}
