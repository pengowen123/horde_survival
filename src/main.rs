// TODO: Fix attack_ranged_projectile (fix angle and make sure the physics is correct)
#![feature(const_fn)]

// Graphics
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate cgmath;

// Windows API
extern crate user32;
extern crate winapi;

// Logging
#[macro_use]
extern crate log;
extern crate log_panics;

// Misc
extern crate rand;
extern crate time;

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

use gfx::Device;
use glutin::Event;
use user32::FindWindowW;

use utils::*;
use entity::*;
use world::*;
use player::*;
use consts::*;
use hslog::*;
use hsgraphics::*;
use gamestate::GameState;
use map::*;

use std::ptr;
use std::time::{Instant, Duration};

fn main() {
    // Initialize logger
    hslog::init();
    log_panics::init();

    info!("Building window...");

    // Window setup
    // TODO: Add graphics options
    let builder = glutin::WindowBuilder::new()
        .with_title(WINDOW_NAME)
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);

    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let hwnd = unsafe {
        let name = convert_str(WINDOW_NAME);

        FindWindowW(ptr::null(), name.as_ptr())
    };

    info!("Initializing game...");

    // Game state
    let player_entity_id = 0;
    let map = Map::new(0.0);
    let player = Player::new(player_entity_id, 0, Class::Warrior);
    let mut game = GameState::new(player, map, Coords::origin(), Team::Players);

    // Use this for testing the game
    game.spawn_entity(Entity::zombie(Coords::new(-10.0, 0.0, 5.0), 0, Team::Monsters));
    game.spawn_entity(Entity::zombie(Coords::new(-10.0, 0.0, -5.0), 0, Team::Monsters));
    game.spawn_entity(Entity::zombie(Coords::new(-10.0, 0.0, 0.0), 0, Team::Monsters));
    game.entities[0].coords = Coords::new(10.0, 0.0, 0.0);
    game.entities[1].coords = Coords::new(0.0, 0.0, 5.0);
    game.entities[2].coords = Coords::new(10.0, 0.0, -5.0);
    game.entities[3].coords = Coords::new(-10.0, 0.0, 0.0);
    game.entities[0].armor[0] = consts::items::armor::HEAL;
    game.entities[1].current_weapon = TEST_BOW;
    game.entities[0].current_weapon = TEST_BOW;

    // Graphics state
    let mut graphics = GraphicsState::new(&mut factory);

    // Used for keeping TPS below a certain value
    let mut time;
    let mut sleeping_until = Instant::now();
    let expected_elapsed = Duration::from_millis(1_000_000_000 / TPS / 1_000_000);

    // Event loop

    let mut events = window.poll_events();

    info!("Done");

    'main: loop {
        time = Instant::now();
        let event = events.next();

        if let Some(event) = event {
            // User input
            match event {
                Event::KeyboardInput(state, scan_code, key) => {
                    let player = &mut game.player;

                    hscontrols::handle_keyboard_input(key,
                                                      state,
                                                      scan_code,
                                                      &mut game.entities,
                                                      player,
                                                      &window);
                },
                Event::MouseInput(state, button) => {
                    match button {
                        glutin::MouseButton::Left => {
                            game.player.left_click = state == glutin::ElementState::Pressed;
                        },
                        _ => {},
                    }
                },
                Event::Closed => {
                    info!("Closed Horde Survival");
                    break 'main;
                }
                _ => {},
            }
        }

        encoder.clear(&main_color, CLEAR_COLOR);
        graphics.encode_objects(&mut encoder);
        encoder.flush(&mut device);

        if let Err(e) = window.swap_buffers() {
            error!("Failed to swap buffers: {}", e);
        }

        device.cleanup();

        // Skip game loop until the tick has finished
        if Instant::now() < sleeping_until {
            continue;
        }

        // Game loop

        game.player.update_cooldowns();

        // Scoped for other entity updates
        {
            // Player updates
            let player = &mut game.player;

            if player.left_click && player.capture_cursor {
                player.gold += player.bounty * try_attack(player.entity_id,
                                                          &mut game.entities,
                                                          &mut game.next_entity_id,
                                                          player);
            }

            let player_entity = unwrap_or_log!(game.entities.iter_mut().find(|e| e.id == player.entity_id),
                                               "Player entity disappeared");

            update_player(player_entity,
                          &mut player.dead,
                          player.move_forward,
                          player.move_left,
                          player.move_right,
                          player.move_backward,
                          &player.mouse);
        }

        for i in 0..game.entities.len() {
            update_entity(&mut game.entities, i, &game.map, &mut game.player, &mut game.next_entity_id);
        }

        filter_entities(&mut game.entities);

        // Bind direction to mouse
        if game.player.capture_cursor {
            // TODO: Change center_mouse to use glutin::Window methods rather than winapi
            hscontrols::center_mouse(&mut game.player.mouse,
                                     hwnd,
                                     &mut graphics.window_position,
                                     &mut graphics.window_center);
        }

        // Update minimap
        graphics.update_minimap(&game.entities);
        graphics.update_minimap_objects(&mut factory, &main_color);

        // Set duration to skip game loop for
        let current = Instant::now();
        let elapsed = current - time;

        if elapsed < expected_elapsed {
            sleeping_until = current + (expected_elapsed - elapsed);
        }
    }
}
