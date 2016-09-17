use winapi::POINT;

use consts::{DEFAULT_MOUSE_SENSITIVITY, BASE_MOVESPEED};
use hscontrols::*;
use world::*;
use entity::Entity;
use items::WeaponType;

pub fn update_player(player: &mut Entity,
                     dead: &mut bool,
                     move_forward: bool,
                     move_left: bool,
                     move_right: bool,
                     move_backward: bool,
                     mouse: &POINT,
                     player_direction: &mut (f64, f64),
                     player_coords: &mut Coords) {

    *dead = player.is_dead();


    if move_forward || move_left || move_right || move_backward {
        let speed = player.movespeed_mods.iter().fold(BASE_MOVESPEED, |acc, x| acc * x.value);

        // NOTE: move_left and move_right are swapped to fix a camera issu3
        let offset = get_movement_offset(move_forward, move_right, move_left, move_backward);

        player.move_forward(offset);
        player_coords.move_forward(Direction(player_direction.1 + offset).wrap().0, speed);
    }
    
    let x = &mut player.direction.0;
    let y = &mut player.direction.1;

    // NOTE: Values get multiplied by -1.0 to invert controls
    let mut move_x = DEFAULT_MOUSE_SENSITIVITY * mouse.y as f64;
    let move_y = DEFAULT_MOUSE_SENSITIVITY * mouse.x as f64;

    player_direction.0 += move_x;
    player_direction.1 += move_y * -1.0;

    // I don't know why this is necessary, but it works
    if let WeaponType::RangedProjectile = player.current_weapon.weapon_type {
        move_x *= -1.0;
    }

    *x += move_x;
    *y += move_y * -1.0;

    *y = Direction(*y).wrap().0;
    if *x < 0.0 { *x = 0.0; } else if *x > 180.0 { *x = 180.0; }
}
