use glutin::Window;

use gamestate::GameState;
use player::AbilityID;
use hsgraphics::GraphicsState;
use entity::update_player;
use hscontrols::center_mouse;

/// Updates player related things such as ability cooldowns
pub fn update_player_state(game: &mut GameState) {
    // Update player cooldowns
    game.player.update_cooldowns();

    let casts;

    // Scoped for ability casts
    {
        let player = &mut game.player;
        // Get player entity
        let player_entity = find_player_entity!(game.entities.iter_mut(), player);

        if player.left_click {
            player_entity.attack = true;
        }

        // Update player entity, and get a list of ability casts
        casts = update_player(player_entity, player);
    }

    // Cast abilities if requested
    if casts[0] {
        game.player.cast_ability(&mut game.entities, AbilityID::A);
    }
    if casts[1] {
        game.player.cast_ability(&mut game.entities, AbilityID::B);
    }
    if casts[2] {
        game.player.cast_ability(&mut game.entities, AbilityID::C);
    }
    if casts[3] {
        game.player.cast_ability(&mut game.entities, AbilityID::D);
    }
}

/// Updates player related things that aren't TPS bound
pub fn update_player_non_tps_bound(game: &mut GameState,
                                   graphics: &mut GraphicsState,
                                   window: &Window) {

    // NOTE: Updating the camera here (faster than TPS) might not have any benefit over doing it in
    //       update_player_state, as the mouse input, which controls the camera, is bound to TPS
    graphics.update_camera(game.player.camera.clone());
    center_mouse(graphics, &mut game.player.mouse, window);
}
