use piston::input::*;
use piston::window::AdvancedWindow;
use glutin_window::GlutinWindow;

use player::*;
use entity::Entity;

pub fn handle_input(input: Input,
                    entities: &mut Vec<Entity>,
                    player: &mut Player,
                    capture_cursor: &mut bool,
                    window: &mut GlutinWindow,
                    move_forward: &mut bool,
                    move_left: &mut bool,
                    move_backward: &mut bool,
                    move_right: &mut bool,
                    left_click: &mut bool,
                    dead: bool) {
    match input {
        Input::Press(button) => {
            match button {
                Button::Mouse(button) => {
                    match button {
                        MouseButton::Left => {
                            *left_click = true;
                        },
                        _ => {},
                    }
                },
                Button::Keyboard(key) => {
                    match key {
                        // GUI
                        Key::Escape => {
                            *capture_cursor = !*capture_cursor;
                            window.set_capture_cursor(*capture_cursor);
                        },
                        // Movement
                        Key::W => *move_forward = true,
                        Key::A => *move_left = true,
                        Key::S => *move_backward = true,
                        Key::D => *move_right = true,
                        // Abilities
                        Key::D1 => {
                            if player.current_cooldowns[0] > 0 {
                                info!("Ability 0: on cooldown");
                            } else if dead {
                                info!("Ability 0: dead");
                            } else {
                                player.ability_0(entities);
                            }
                        },
                        Key::D2 => {
                            if player.current_cooldowns[1] > 0 {
                                info!("Ability 1: on cooldown");
                            } else if dead {
                                info!("Ability 1: dead");
                            } else {
                                player.ability_1(entities);
                            }
                        },
                        Key::D3 => {
                            if player.current_cooldowns[2] > 0 {
                                info!("Ability 2: on cooldown");
                            } else if dead {
                                info!("Ability 2: dead");
                            } else {
                                player.ability_2(entities);
                            }
                        },
                        Key::D4 => {
                            if player.current_cooldowns[3] > 0 {
                                info!("Ability 3: on cooldown");
                            } else if dead {
                                info!("Ability 3: dead");
                            } else {
                                player.ability_3(entities);
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
                            *left_click = false;
                        },
                        _ => {},
                    }
                },
                Button::Keyboard(key) => {
                    match key {
                        Key::W => *move_forward = false,
                        Key::A => *move_left = false,
                        Key::S => *move_backward = false,
                        Key::D => *move_right = false,
                        _ => {},
                    }
                }
                _ => {},
            }
        },
        _ => {},
    }
}
