#![feature(const_fn)]

mod utils;
mod entity;
mod world;
mod player;
mod consts;

extern crate piston_window;
extern crate user32;
extern crate winapi;

use user32::{GetCursorPos, SetCursorPos, FindWindowW};
use winapi::POINT;
use piston_window::*;

use utils::*;
use entity::*;
use world::*;
use player::*;
use consts::*;

use std::ptr;

fn main() {
    // Initialize entity list
    let mut entities = vec![
        Entity::new(0, 100.0, Coords::origin(), EntityType::Player, false),
        Entity::new(1, 50.0, Coords::origin(), EntityType::Zombie, true),
        Entity::new(2, 50.0, Coords::origin(), EntityType::Zombie, true),
    ];

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

    // Movement state variables
    let mut move_forward = false;
    let mut move_left = false;
    let mut move_right = false;
    let mut move_backward = false;

    // Mouse state variables
    let mut capture_cursor = false;
    let mut mouse = POINT { x: 0, y: 0 };

    // Window state variables
    let mut window_position = (0, 0);
    let mut center;

    // Event loop
    while let Some(event) = events.next(&mut window) {
        // User input

        match event {
            Event::Input(input) => {
                match input {
                    Input::Press(button) => {
                        match button {
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

        // Game loop

        player.update_cooldowns();

        entities = entities.into_iter().filter(|e| {
            if e.is_enemy { !e.is_dead() } else { true }
        }).collect();

        for entity in &mut entities {
            if entity.id == player.entity_id {
                dead = entity.is_dead();

                if move_forward || move_left || move_right || move_backward {
                    entity.move_forward(get_movement_offset(move_forward,
                                                            move_left,
                                                            move_right,
                                                            move_backward));
                }
                
                let x = &mut entity.direction.0;
                let y = &mut entity.direction.1;

                *x += DEFAULT_MOUSE_SENSITIVITY * mouse.y as f64;
                *y += DEFAULT_MOUSE_SENSITIVITY * mouse.x as f64;

                *x = Direction(*x).wrap().0;
                *y = Direction(*y).wrap().0;
            }

            update_modifiers(&mut entity.damage_mods);
            update_modifiers(&mut entity.as_mods);
            update_modifiers(&mut entity.damage_taken_mods);
            update_modifiers(&mut entity.movespeed_mods);
        }

        // Bind direction to mouse
        if capture_cursor {
            unsafe {
                GetCursorPos(&mut mouse);
            }

            window_position = get_window_position(hwnd, window_position);

            center = (window_position.0 + WINDOW_WIDTH as i32 / 2,
                      window_position.1 + WINDOW_HEIGHT as i32 / 2);

            mouse.x -= center.0;
            mouse.y -= center.1;

            unsafe {
                SetCursorPos(center.0, center.1);
            }
        }
    }
}
