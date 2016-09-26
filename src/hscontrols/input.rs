use glutin::*;

use player::*;
use entity::Entity;

pub fn handle_keyboard_input(key: Option<VirtualKeyCode>,
                             state: ElementState,
                             scan_code: ScanCode,
                             entities: &mut Vec<Entity>,
                             player: &mut Player,
                             window: &Window) {
    
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
                    player.capture_cursor = !player.capture_cursor;
                    if player.capture_cursor {
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
                VirtualKeyCode::W => player.move_forward = player.capture_cursor,
                VirtualKeyCode::A => player.move_left = player.capture_cursor,
                VirtualKeyCode::S => player.move_backward = player.capture_cursor,
                VirtualKeyCode::D => player.move_right = player.capture_cursor,
                // Abilities
                VirtualKeyCode::Key1 => {
                    if player.dead {
                        info!("Ability 0: dead");
                    } else {
                        player.ability_0(entities);
                    }
                },
                VirtualKeyCode::Key2 => {
                    if player.dead {
                        info!("Ability 1: dead");
                    } else {
                        player.ability_1(entities);
                    }
                },
                VirtualKeyCode::Key3 => {
                    if player.dead {
                        info!("Ability 2: dead");
                    } else {
                        player.ability_2(entities);
                    }
                },
                VirtualKeyCode::Key4 => {
                    if player.dead {
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
                VirtualKeyCode::W => player.move_forward = false,
                VirtualKeyCode::A => player.move_left = false,
                VirtualKeyCode::S => player.move_backward = false,
                VirtualKeyCode::D => player.move_right = false,
                _ => {},
            }
        },
    }

    if !player.capture_cursor {
        player.move_left = false;
        player.move_right = false;
        player.move_forward = false;
        player.move_backward = false;
    }
}
