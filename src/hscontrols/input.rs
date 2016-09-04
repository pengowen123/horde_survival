use glutin::*;

use player::*;
use entity::Entity;

pub fn handle_keyboard_input(key: Option<VirtualKeyCode>,
                             state: ElementState,
                             scan_code: ScanCode,
                             entities: &mut Vec<Entity>,
                             player: &mut Player,
                             capture_cursor: &mut bool,
                             window: &Window,
                             move_forward: &mut bool,
                             move_left: &mut bool,
                             move_backward: &mut bool,
                             move_right: &mut bool,
                             dead: bool) {
    
    let key = match key {
        Some(key) => key,
        None => {
            warn!("VirtualKeyCode not found for key: {:?}", scan_code);
            return;
        },
    };

    match state {
        ElementState::Pressed => {
            match key {
                // GUI
                VirtualKeyCode::Escape => {
                    *capture_cursor = !*capture_cursor;
                    if *capture_cursor {
                        if let Err(e) = window.set_cursor_state(CursorState::Hide) {
                            error!("Failed to set cursor state (hide): {:?}", e);
                        }
                    } else {
                        if let Err(e) = window.set_cursor_state(CursorState::Normal) {
                            error!("Failed to set cursor state (show): {:?}", e);
                        }
                    }
                },
                // Movement
                VirtualKeyCode::W => *move_forward = true,
                VirtualKeyCode::A => *move_left = true,
                VirtualKeyCode::S => *move_backward = true,
                VirtualKeyCode::D => *move_right = true,
                // Abilities
                VirtualKeyCode::Key1 => {
                    if player.current_cooldowns[0] > 0 {
                        info!("Ability 0: on cooldown");
                    } else if dead {
                        info!("Ability 0: dead");
                    } else {
                        player.ability_0(entities);
                    }
                },
                VirtualKeyCode::Key2 => {
                    if player.current_cooldowns[1] > 0 {
                        info!("Ability 1: on cooldown");
                    } else if dead {
                        info!("Ability 1: dead");
                    } else {
                        player.ability_1(entities);
                    }
                },
                VirtualKeyCode::Key3 => {
                    if player.current_cooldowns[2] > 0 {
                        info!("Ability 2: on cooldown");
                    } else if dead {
                        info!("Ability 2: dead");
                    } else {
                        player.ability_2(entities);
                    }
                },
                VirtualKeyCode::Key4 => {
                    if player.current_cooldowns[3] > 0 {
                        info!("Ability 3: on cooldown");
                    } else if dead {
                        info!("Ability 3: dead");
                    } else {
                        player.ability_3(entities);
                    }
                },
                _ => {},
            }
        },
        ElementState::Released => {
            match key {
                VirtualKeyCode::W => *move_forward = false,
                VirtualKeyCode::A => *move_left = false,
                VirtualKeyCode::S => *move_backward = false,
                VirtualKeyCode::D => *move_right = false,
                _ => {},
            }
        },
    }
}
