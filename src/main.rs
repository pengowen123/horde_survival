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

// Windows API
// TODO: Find an alternative for linux and osx
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

    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<gfx3d::ColorFormat, gfx3d::DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let hwnd = unsafe {
        let name = convert_str(WINDOW_NAME);

        FindWindowW(ptr::null(), name.as_ptr())
    };

    info!("Initializing game...");

    // Graphics state
    let mut graphics = GraphicsState::new(&mut factory);

    // Game state
    let player_entity_id = 0;
    let map = Map::new(0.0);
    let player = Player::new(player_entity_id, 0, Class::Warrior);
    let mut game = GameState::new(player, map, Coords::origin(), Team::Players);

    // Use this for testing the game
    game.entities[0].coords.x += 5.0;
    game.entities[0].current_weapon = TEST_GUN;
    game.spawn_entity(Entity::zombie(Coords::new(10.0, 0.0, 0.0), 0, Team::Players, 0));
    //game.entities[0].armor[0] = consts::items::armor::HEAL;

    // TODO: Fix performance issues with spawning 100 entities
    //       The problem seems to only exist with controls, not graphics or game updates
    //       It only appears when a certain distance from the entities for some reason (maybe its
    //       the AI?)
    //       While running in release mode reduces the issue, it would still be nice to fix it
    //
    //use rand::Rng;
    //let bounty = game.bounty;
    //for i in 0..100 {
        //let coords = [rand::thread_rng().gen::<f64>() * 5.0; 3];
        //let coords = Coords::new(coords[0], coords[1], coords[2]);
        //let team = Team::Monsters;
        //game.spawn_entity(Entity::zombie(coords, 0, team, bounty));
        //game.entities[i + 1].current_weapon = TEST_WAND;
    //}

    // Used for keeping TPS below a certain value
    let mut time;
    let mut sleeping_until = Instant::now();
    let expected_elapsed = Duration::from_millis(1_000_000_000 / TPS / 1_000_000);

    // NOTE: Remove cube_object when 3d entity objects are implemented
    use hsgraphics::object::object3d::Object3d;

    let cube_object = Object3d::from_slice(&mut factory,
                                           shapes3d::cube([0.0, 0.0, 0.0], 1.0),
                                           main_color.clone(),
                                           main_depth.clone(),
                                           graphics.get_texture(1),
                                           graphics.sampler.clone());

    let floor_object = Object3d::from_slice(&mut factory,
                                            shapes3d::plane(FLOOR_HEIGHT, 1000.0),
                                            main_color.clone(),
                                            main_depth.clone(),
                                            graphics.get_texture(0),
                                            graphics.sampler.clone());

    graphics.add_object3d(floor_object, 0);
    graphics.add_object3d(cube_object, 1);

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

        encoder.clear(&main_color, hsgraphics::CLEAR_COLOR);
        encoder.clear_depth(&main_depth, 1.0);
        graphics.encode_objects3d(&mut encoder);
        graphics.encode_objects2d(&mut encoder);

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
            let direction = player.direction;

            if player.left_click && player.capture_cursor {
                let gold_gained = try_attack(player.entity_id,
                                             &mut game.entities,
                                             &mut game.next_entity_id,
                                             player);

                player.give_gold(gold_gained);
            }

            let player_entity = unwrap_or_log!(game.entities.iter_mut().find(|e| e.id == player.entity_id),
                                               "Player entity disappeared");

            graphics.update_camera(player.coords.clone(), direction);

            update_player(player_entity,
                          &mut player.dead,
                          player.move_forward,
                          player.move_left,
                          player.move_right,
                          player.move_backward,
                          &player.mouse,
                          &mut player.direction,
                          &mut player.coords);
        }

        for i in 0..game.entities.len() {
            update_entity(&mut game.entities, i, &game.map, &mut game.player, &mut game.next_entity_id);
        }

        filter_entities(&mut game.entities);

        // Bind direction to mouse
        if game.player.capture_cursor {
            hscontrols::center_mouse(&mut game.player.mouse,
                                     hwnd,
                                     &mut graphics.window_position,
                                     &mut graphics.window_center);
        } else {
            game.player.mouse = winapi::POINT { x: 0, y: 0 };
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
