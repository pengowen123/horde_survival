use winapi::POINT;

use consts::controls::*;
use world::Direction;
use entity::Entity;
use hscontrols::*;

pub fn update_player(player: &mut Entity,
                     dead: &mut bool,
                     move_forward: bool,
                     move_left: bool,
                     move_right: bool,
                     move_backward: bool,
                     mouse: &POINT) {

    *dead = player.is_dead();

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

    *y = Direction(*y).wrap().0;

    if *x < 0.0 {
        *x = 0.0;
    } else if *x > 180.0 {
        *x = 180.0;
    }
}
