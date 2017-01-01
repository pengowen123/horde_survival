use consts::{DEFAULT_MOUSE_SENSITIVITY, BASE_MOVESPEED};
use world::*;
use entity::{Entity, apply};
use player::Player;
use items::WeaponType;

pub fn update_player(entity: &mut Entity, player: &mut Player) -> [bool; 4] {

    player.dead = entity.is_dead();

    if player.input.movement_key_pressed() {
        let speed = apply(&entity.movespeed_mods, BASE_MOVESPEED);

        let offset = player.input.movement_offset();

        entity.move_forward(offset);
        player.coords.move_forward(Direction(player.direction.1 + offset).wrap().0, speed);
    }

    player.coords.y = entity.coords.y;

    let x = &mut entity.direction.0;
    let y = &mut entity.direction.1;

    let mut move_x = DEFAULT_MOUSE_SENSITIVITY * player.mouse.1 as f64;
    let move_y = DEFAULT_MOUSE_SENSITIVITY * player.mouse.0 as f64;

    player.direction.0 += move_x;
    player.direction.1 += move_y * -1.0;
    player.direction.1 = Direction(player.direction.1).wrap().0;

    if player.direction.0 < 1.0 {
        player.direction.0 = 1.0
    } else if player.direction.0 > 179.0 {
        player.direction.0 = 179.0
    }

    // FIXME: Investigate why this is necessary
    //        If weapon type is RangedProjectile, the vertical controls get inversed. This is to
    //        counteract that.
    if let WeaponType::RangedProjectile = entity.current_weapon.weapon_type {
        move_x *= -1.0;
    }

    *x += move_x;
    *y += move_y * -1.0;

    *y = Direction(*y).wrap().0;
    if *x < 1.0 {
        *x = 1.0;
    } else if *x > 179.0 {
        *x = 179.0;
    }

    [entity.animations.is_casting(1),
     entity.animations.is_casting(2),
     entity.animations.is_casting(3),
     entity.animations.is_casting(4)]
}
