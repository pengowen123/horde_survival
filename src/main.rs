#![feature(const_fn)]

mod utils;
mod entity;
mod items;
mod world;
mod player;
mod consts;

extern crate piston_window;
extern crate user32;
extern crate winapi;
extern crate time;

use user32::{GetCursorPos, SetCursorPos, FindWindowW};
use winapi::POINT;
use piston_window::*;
use time::precise_time_ns;

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
        Entity::new(0, 100.0, Coords::origin(), EntityType::Player, false, false, (90.0, 0.0), 0),
        Entity::new(1, 50.0, Coords::origin(), EntityType::Zombie, true, false, (90.0, 0.0), 0),
        Entity::new(2, 50.0, Coords::origin(), EntityType::Zombie, true, false, (90.0, 0.0), 0),
    ];

    entities[0].current_weapon = WEAPON_TEST_WAND;

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
    let mut center;

    // Timer (keeps tps equal to or less than some constant)
    let mut sleeping_until = Instant::now();
    let expected_elapsed = 1_000_000_000 / TPS;

    // Event loop
    while let Some(event) = events.next(&mut window) {
        let time = precise_time_ns();

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
                    player.move_forward(get_movement_offset(move_forward,
                                                            move_left,
                                                            move_right,
                                                            move_backward));
                }
                
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
            let entity_type;
            let is_enemy;
            let coords;

            // Scoped for other entity updates
            {
                let entity = &mut entities[i];
                entity_type = entity.entity_type.clone();
                is_enemy = entity.is_enemy;
                coords = entity.coords.clone();

                update_modifiers(&mut entity.damage_mods);
                update_modifiers(&mut entity.as_mods);
                update_modifiers(&mut entity.damage_taken_mods);
                update_modifiers(&mut entity.movespeed_mods);
                
                if entity.attack_animation > 0 {
                    entity.attack_animation -= 1;
                }

                if entity.lifetime > 1 {
                    entity.lifetime -= 1;
                }
            }

            match entity_type {
                EntityType::FlyingBallLinear => {
                    let hit;

                    match entities.iter()
                                  .enumerate()
                                  .filter_map(|(i, e)| {
                                      if e.is_enemy != is_enemy && e.coords.in_radius(&coords, RANGED_LINEAR_RADIUS) {
                                          Some(i)
                                      } else {
                                          None
                                      }
                                  })
                                  .nth(0) {
                        Some(e) => {
                            let damage;
                            let id;
                            hit = true;
                            // Scoped for attack_entity call
                            {
                                let entity = &entities[i];
                                let weapon_damage = entity.current_weapon.damage;
                                damage = entity.damage_mods.iter().fold(weapon_damage, |acc, x| acc * x.value);
                                id = entity.id;
                            }

                            if entities[e].damage(damage) && id == player.entity_id {
                                player.gold += player.bounty;
                            }
                        },
                        None => {
                            hit = false;
                        },
                    }
                    
                    let entity = &mut entities[i];

                    if hit {
                        entity.lifetime = 1;
                    }

                    entity.coords.move_3d(entity.direction, entity.as_mods[0].value);
                },
                EntityType::FlyingBallArc => {
                },
                _ => {},
            }
        }

        entities = entities.iter().cloned().filter(|e| {
            (if e.is_enemy { !e.is_dead() } else { true }) && !(e.lifetime == 1)
        }).collect();

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

        // Ensure TPS is less than or equal to some constant
        let elapsed = precise_time_ns() - time;

        if elapsed < expected_elapsed {
            let difference = expected_elapsed - elapsed;
            sleeping_until = Instant::now() + Duration::from_millis(difference / 1_000_000);
        }
    }
}
