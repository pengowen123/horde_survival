use glutin::*;

use player::*;
use entity::Entity;

/// Handles keyboard input
pub fn handle_keyboard_input(key: Option<VirtualKeyCode>,
                             state: ElementState,
                             scan_code: ScanCode,
                             entities: &mut Vec<Entity>,
                             player: &mut Player) {

    let key = if let Some(key) = key {
        key
    } else {
        warn!("VirtualKeyCode not found for key: {:?}", scan_code);
        return;
    };

    match state {
        // TODO: Allow for customization of these keys
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
                        player.cast_ability(entities, AbilityID::A);
                    }
                }
                VirtualKeyCode::Key2 => {
                    if player.dead {
                        info!("Ability 1: dead");
                    } else {
                        player.cast_ability(entities, AbilityID::B);
                    }
                }
                VirtualKeyCode::Key3 => {
                    if player.dead {
                        info!("Ability 2: dead");
                    } else {
                        player.cast_ability(entities, AbilityID::C);
                    }
                }
                VirtualKeyCode::Key4 => {
                    if player.dead {
                        info!("Ability 3: dead");
                    } else {
                        player.cast_ability(entities, AbilityID::D);
                    }
                }
                _ => {}
            }
        }
        // If movement keys are released, stop moving
        ElementState::Released => {
            match key {
                VirtualKeyCode::W => player.input.forward = false,
                VirtualKeyCode::A => player.input.left = false,
                VirtualKeyCode::S => player.input.back = false,
                VirtualKeyCode::D => player.input.right = false,
                _ => {}
            }
        }
    }
}
