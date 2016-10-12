use glutin::*;

use player::*;
use entity::Entity;

pub fn handle_keyboard_input(key: Option<VirtualKeyCode>,
                             state: ElementState,
                             scan_code: ScanCode,
                             entities: &mut Vec<Entity>,
                             player: &mut Player) {
    
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
                // Movement
                VirtualKeyCode::W => player.input.forward = true,
                VirtualKeyCode::A => player.input.left = true,
                VirtualKeyCode::S => player.input.back = true,
                VirtualKeyCode::D => player.input.right = true,
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
                VirtualKeyCode::W => player.input.forward = false,
                VirtualKeyCode::A => player.input.left = false,
                VirtualKeyCode::S => player.input.back = false,
                VirtualKeyCode::D => player.input.right = false,
                _ => {},
            }
        },
    }
}
