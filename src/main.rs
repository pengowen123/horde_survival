#![feature(const_fn)]

// Graphics
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate gfx_app;
extern crate glutin;
extern crate cgmath;

// Windows API
// NOTE: These may be removed soon
extern crate user32;
extern crate winapi;

// Logging
#[macro_use]
extern crate log;
extern crate simplelog;

// Game engine
extern crate piston;

// Misc
extern crate rand;

#[macro_use]
mod log_utils;
mod utils;
mod entity;
mod items;
mod world;
mod player;
mod consts;
mod hscontrols;
mod hsgraphics;
mod map;

use gfx::traits::FactoryExt;
use gfx::Device;

use glutin::Event;
use user32::FindWindowW;
use winapi::POINT;
use simplelog::{FileLogger, LogLevelFilter};

use utils::*;
use entity::*;
use world::*;
use player::*;
use consts::*;
use log_utils::*;
use hsgraphics::*;
use map::*;

use std::ptr;
use std::time::{Instant, Duration};
use std::fs::File;

fn main() {
    // Initialize logger
    match FileLogger::init(LogLevelFilter::max(), File::create("log.txt").expect("Failed to initialize log file")) {
        Ok(_) => {},
        Err(e) => panic!("Failed to initialize logger: {}", e),
    }

    // Initialize entity list
    let mut entities = vec![
        Entity::new(0, 100.0, 100.0, Coords::new(0.0, 0.0, 0.0), EntityType::Player, Team::Players, IsDummy::False, (90.0, 0.0), 0, HasGravity::True, HasAI::False),
        // Use this entity for testing
        //Entity::new(1, 100.0, 100.0, Coords::origin(), EntityType::Zombie, Team::Monsters, IsDummy::False, (90.0, 0.0), 0, HasGravity::True, HasAI::True),
    ];

    entities[0].current_weapon = WEAPON_LIGHTNING_SWORD_2;
    //entities[1].current_weapon = WEAPON_TEST_WAND;

    let mut next_id = entities.len();

    // Player
    // TODO: Add multiplayer support
    let mut player = Player::new(0, 0, Class::Warrior);
    let mut dead = false;

    // Window setup
    // TODO: Add graphics options
    let builder = glutin::WindowBuilder::new()
        .with_title(WINDOW_NAME)
        .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);

    let (window, mut device, mut factory, main_color, color_depth) =
        gfx_window_glutin::init::<ColorFormat, ColorDepth>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    // Begin test code

    let pso = match factory.create_pipeline_simple(
        include_bytes!("hsgraphics/shader/triangle_150.glslv"),
        include_bytes!("hsgraphics/shader/triangle_150.glslf"),
        pipe::new(),
    ) {
        Ok(x) => x,
        Err(e) => panic!("{}", e),
    };

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SQUARE, ());
    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };

    // End test code

    let hwnd = unsafe {
        let name = convert_str(WINDOW_NAME);

        FindWindowW(ptr::null(), name.as_ptr())
    };

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

    // Graphics state
    let mut gfx_state = GraphicsState::new();

    // Event loop
    let mut events = window.poll_events();

    // Used for keeping TPS below a certain value
    let mut time;
    let mut sleeping_until = Instant::now();
    let expected_elapsed = Duration::from_millis(1_000_000_000 / TPS / 1_000_000);

    // Event loop
    'main: loop {
        time = Instant::now();
        let event = events.next();

        if let Some(event) = event {
            // User input
            match event {
                Event::KeyboardInput(state, scan_code, key) => {
                    hscontrols::handle_keyboard_input(key,
                                                      state,
                                                      scan_code,
                                                      &mut entities,
                                                      &mut player,
                                                      &mut capture_cursor,
                                                      &window,
                                                      &mut move_forward,
                                                      &mut move_left,
                                                      &mut move_backward,
                                                      &mut move_right,
                                                      dead);
                },
                Event::MouseInput(state, button) => {
                    match button {
                        glutin::MouseButton::Left => {
                            left_click = state == glutin::ElementState::Pressed;
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

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
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

        player.update_cooldowns();

        // Scoped for other entity updates
        {
            // Player updates
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
            // TODO: Change center_mouse to use glutin::Window methods rather than winapi
            hscontrols::center_mouse(&mut mouse, hwnd, &mut window_position, &mut center);
        }

        // Set duration to skip game loop for
        let current = Instant::now();
        let elapsed = current - time;

        if elapsed < expected_elapsed {
            sleeping_until = current + (expected_elapsed - elapsed);
        }
    }
}
