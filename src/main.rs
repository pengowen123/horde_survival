#![feature(const_fn)]

mod utils;
mod entity;
mod items;
mod world;
mod player;
mod consts;
mod flags;
mod hscontrols;

extern crate piston_window;
extern crate user32;
extern crate winapi;

use user32::FindWindowW;
use winapi::POINT;
use piston_window::*;

use flags::*;
use utils::*;
use entity::*;
use world::*;
use player::*;
use consts::*;

use std::ptr;
use std::time::{Duration, Instant};

fn main() {
    // Initialize entity list
    let mut entities = vec![
        Entity::new(0, 100.0, Coords::new(0.0, 100.0, 0.0), EntityType::Player, false, false, (90.0, 0.0), 0, HasGravity::True),
        //Entity::new(1, 50.0, Coords::origin(), EntityType::Zombie, true, false, (90.0, 0.0), 0, HasGravity::False),
        //Entity::new(2, 50.0, Coords::origin(), EntityType::Zombie, true, false, (90.0, 0.0), 0, HasGravity::False),
    ];

    entities[0].current_weapon = WEAPON_TEST_BOW;

    let mut next_id = entities.len();

    // Player
    // TODO: Add multiplayer support
    let mut player = Player::new(0, 0, Class::Warrior);
    let mut dead = false;

    // Window setup
    // TODO: Add graphics options
    let mut window: PistonWindow = WindowSettings::new(WINDOW_NAME, (WINDOW_WIDTH, WINDOW_HEIGHT))
        .build()
        .expect("Failed to build window");

    let hwnd = unsafe {
        let name = convert_str(WINDOW_NAME);

        FindWindowW(ptr::null(), name.as_ptr())
    };

    let mut events = window.events();
    
    events.set_ups(30);
    events.set_max_fps(30);

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
                match input {
                    Input::Press(button) => {
                        match button {
                            Button::Mouse(button) => {
                                match button {
                                    MouseButton::Left => {
                                        left_click = true;
                                    },
                                    _ => {},
                                }
                            },
                            Button::Keyboard(key) => {
                                match key {
                                    // GUI
                                    Key::Escape => {
                                        capture_cursor = !capture_cursor;
                                        window.set_capture_cursor(capture_cursor);
                                    },
                                    // Movement
                                    Key::W => move_forward = true,
                                    Key::A => move_left = true,
                                    Key::S => move_backward = true,
                                    Key::D => move_right = true,
                                    // Abilities
                                    Key::D1 => {
                                        if player.current_cooldowns[0] > 0 {
                                            println!("Ability 0: on cooldown");
                                        } else if dead {
                                            println!("Ability 0: dead");
                                        } else {
                                            player.ability_0(&mut entities);
                                        }
                                    },
                                    Key::D2 => {
                                        if player.current_cooldowns[1] > 0 {
                                            println!("Ability 1: on cooldown");
                                        } else if dead {
                                            println!("Ability 1: dead");
                                        } else {
                                            player.ability_1(&mut entities);
                                        }
                                    },
                                    Key::D3 => {
                                        if player.current_cooldowns[2] > 0 {
                                            println!("Ability 2: on cooldown");
                                        } else if dead {
                                            println!("Ability 2: dead");
                                        } else {
                                            player.ability_2(&mut entities);
                                        }
                                    },
                                    Key::D4 => {
                                        if player.current_cooldowns[3] > 0 {
                                            println!("Ability 3: on cooldown");
                                        } else if dead {
                                            println!("Ability 3: dead");
                                        } else {
                                            player.ability_3(&mut entities);
                                        }
                                    },
                                    _ => {
                                    },
                                }
                            },
                            _ => {},
                        }
                    },
                    Input::Release(button) => {
                        match button {
                            Button::Mouse(button) => {
                                match button {
                                    MouseButton::Left => {
                                        left_click = false;
                                    },
                                    _ => {},
                                }
                            },
                            Button::Keyboard(key) => {
                                match key {
                                    Key::W => move_forward = false,
                                    Key::A => move_left = false,
                                    Key::S => move_backward = false,
                                    Key::D => move_right = false,
                                    _ => {},
                                }
                            }
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            _ => {},

        }

        // Idle while waiting for tick to finish
        if sleeping_until > Instant::now() {
            continue;
        }

        // Game loop

        player.update_cooldowns();

        // Scoped for entity update loop
        {
            // Scoped for try_attack call
            {
                let player = entities.iter_mut().find(|e| e.id == player.entity_id).expect("Player entity disappeared");

                dead = player.is_dead();

                if move_forward || move_left || move_right || move_backward {
                    player.move_forward(hscontrols::get_movement_offset(move_forward,
                                                            move_left,
                                                            move_right,
                                                            move_backward));
                }
                
                println!("player coords: {:?}", player.coords);
                let x = &mut player.direction.0;
                let y = &mut player.direction.1;

                *x += DEFAULT_MOUSE_SENSITIVITY * mouse.y as f64;
                *y += DEFAULT_MOUSE_SENSITIVITY * mouse.x as f64;

                *x = Direction(*x).wrap().0;
                *y = Direction(*y).wrap().0;
            }

            if left_click && capture_cursor {
                player.gold += player.bounty * try_attack(player.entity_id, &mut entities, &mut next_id);
            }
        }

        for i in 0..entities.len() {
            update_entity(&mut entities, i, &map, &mut player);
        }

        filter_entities(&mut entities);

        // Bind direction to mouse
        if capture_cursor {
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
