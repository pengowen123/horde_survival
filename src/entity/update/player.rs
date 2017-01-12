use consts::{DEFAULT_MOUSE_SENSITIVITY, BASE_MOVESPEED};
use world::*;
use entity::{Entity, apply};
use player::Player;
use items::WeaponType;
use utils::clamp;

/// Updates the player's state, and returns which abilities they casted
pub fn update_player(entity: &mut Entity, player: &mut Player) -> [bool; 4] {
    let camera_direction = &mut player.camera.direction;

    // Update death flag
    player.dead = entity.is_dead();

    // Apply movement key presses, if any
    if player.input.movement_key_pressed() {
        let speed = apply(&entity.movespeed_mods, BASE_MOVESPEED);

        let offset = player.input.movement_offset();

        entity.move_forward(offset);
        player.camera.coords.move_forward(Direction(camera_direction.1 + offset).wrap().0, speed);
    }

    // Set the camera height
    player.camera.coords.y = entity.coords.y;

    let x = &mut entity.direction.0;
    let y = &mut entity.direction.1;

    // Calculate direction changes
    let mut move_x = DEFAULT_MOUSE_SENSITIVITY * player.mouse.1 as f64;
    let move_y = DEFAULT_MOUSE_SENSITIVITY * player.mouse.0 as f64;

    // Apply the changes to camera direction
    camera_direction.0 += move_x;
    camera_direction.1 += move_y * -1.0;
    camera_direction.1 = Direction(camera_direction.1).wrap().0;

    // Clamp camera direction
    camera_direction.0 = clamp(camera_direction.0, 1.0, 179.0);

    // FIXME: Investigate why this is necessary
    //        If weapon type is RangedProjectile, the vertical controls get inversed. This is to
    //        counteract that.
    if let WeaponType::RangedProjectile = entity.current_weapon.weapon_type {
        move_x *= -1.0;
    }

    // Apply the changes to entity direction (some inverted as graphical and game code handle
    // directions slightly differently)
    *x += move_x;
    *y += move_y * -1.0;

    // Wrap the Y direction on 360.0 degrees
    *y = Direction(*y).wrap().0;
    // Clamp entity direction
    *x = clamp(*x, 1.0, 179.0);

    // Return which abilities are casting
    [entity.animations.is_playing(1),
     entity.animations.is_playing(2),
     entity.animations.is_playing(3),
     entity.animations.is_playing(4)]
}
