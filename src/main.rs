#![feature(const_fn)]

#[macro_use]
extern crate log;
extern crate piston_window;
extern crate user32;
extern crate winapi;
extern crate rand;
extern crate simplelog;
extern crate glium;
extern crate glutin_window;
extern crate piston;

#[macro_use]
mod log_utils;
mod utils;
mod entity;
mod items;
mod world;
mod player;
mod consts;
mod hscontrols;
mod map;
mod hsgraphics;

use piston::window::WindowSettings;
use piston::event_loop::Events;
use piston::input::Event;
use glutin_window::GlutinWindow;
use user32::FindWindowW;
use winapi::POINT;
use simplelog::{FileLogger, LogLevelFilter};
use glium::*;

use utils::*;
use entity::*;
use world::*;
use player::*;
use consts::*;
use log_utils::*;
use hsgraphics::*;
use map::*;

use std::ptr;
use std::time::{Duration, Instant};
use std::fs::File;

fn main() {
    // Initialize logger
    match FileLogger::init(LogLevelFilter::max(), File::create("log.txt").expect("Failed to initialize log file")) {
        Ok(_) => {},
        Err(e) => panic!("Failed to initialize logger: {}", e),
    }

    // Initialize entity list
    let mut entities = vec![
        Entity::new(0, 100.0, 100.0, Coords::new(10.0, 0.0, 0.0), EntityType::Player, Team::Players, IsDummy::False, (90.0, 0.0), 0, HasGravity::True, HasAI::False),
        // Use this entity for testing
        Entity::new(1, 100.0, 100.0, Coords::origin(), EntityType::Zombie, Team::Monsters, IsDummy::False, (90.0, 0.0), 0, HasGravity::True, HasAI::True),
    ];

    entities[0].current_weapon = WEAPON_LIGHTNING_SWORD_2;
    entities[1].current_weapon = WEAPON_TEST_WAND;

    let mut next_id = entities.len();

    // Player
    // TODO: Add multiplayer support
    let mut player = Player::new(0, 0, Class::Warrior);
    let mut dead = false;

    // Window setup
    // TODO: Add graphics options
    let window_settings = WindowSettings::new(WINDOW_NAME, (WINDOW_WIDTH, WINDOW_HEIGHT));
    let mut window = unwrap_or_log!(GlutinWindow::new(&window_settings), "Failed to build window");

    let hwnd = unsafe {
        let name = convert_str(WINDOW_NAME);

        FindWindowW(ptr::null(), name.as_ptr())
    };

    let mut events = window.events();

    // Map
    // TODO: Add multiple maps
    let map = Map::new(0.0);

    // Movement state variables
    let mut move_forward = false;
    let mut move_left = false;
    let mut move_right = false;
    let mut move_backward = false;

    // Mouse state variables
    let mut capture_cursor = false;
    let mut mouse = POINT { x: 0, y: 0 };
    let mut left_click = false;

    // Window state variables
    let mut window_position = (0, 0);
    let mut center = (0, 0);

    // Timer (keeps tps equal to or less than some constant)
    let mut sleeping_until = Instant::now();
    let expected_elapsed = Duration::from_millis(1_000_000_000 / TPS / 1_000_000);

    // Event loop
    while let Some(event) = events.next(&mut window) {
        let time = Instant::now();

        // User input
        match event {
            Event::Input(input) => {
                // TODO: move this code to hscontrols::input::handle_input_event
                hscontrols::handle_input(input,
                                         &mut entities,
                                         &mut player,
                                         &mut capture_cursor,
                                         &mut window,
                                         &mut move_forward,
                                         &mut move_left,
                                         &mut move_backward,
                                         &mut move_right,
                                         &mut left_click,
                                         dead);
            },
            _ => {},

        }

        // Idle while waiting for tick to finish
        if sleeping_until > Instant::now() {
            continue;
        }

        // Game loop

        player.update_cooldowns();

        // Scoped for other entity updates
        {
            if left_click && capture_cursor {
                player.gold += player.bounty * try_attack(player.entity_id, &mut entities, &mut next_id, &mut player);
            }

            let player = unwrap_or_log!(entities.iter_mut().find(|e| e.id == player.entity_id),
                                        "Player entity disappeared");

            update_player(player,
                          &mut dead,
                          move_forward,
                          move_left,
                          move_right,
                          move_backward,
                          &mouse);
        }

        for i in 0..entities.len() {
            update_entity(&mut entities, i, &map, &mut player, &mut next_id);
        }

        filter_entities(&mut entities);

        // Bind direction to mouse
        if capture_cursor {
            // NOTE: If this way of centering the mouse causes problems, try using methods from
            //       AdvancedWindow, as they might be working now
            hscontrols::center_mouse(&mut mouse, hwnd, &mut window_position, &mut center);
        }

        // Ensure TPS is less than or equal to some constant
        // NOTE: If expected_elapsed exceeds 1 second, bad things can happen
        let elapsed = time.elapsed();

        if elapsed < expected_elapsed {
            let difference = expected_elapsed - elapsed;
            sleeping_until = Instant::now() + difference;
        }
    }
}
