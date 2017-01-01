use glutin::Window;

use gamestate::GameState;
use hsgraphics::GraphicsState;
use entity::update_player;
use hslog::CanUnwrap;
use hscontrols::center_mouse;

// Updates player related things such as ability cooldowns
pub fn update_player_state(game: &mut GameState, graphics: &mut GraphicsState) {
    game.player.update_cooldowns();

    let casts;

    // Scoped for ability casts
    {
        let player = &mut game.player;
        let direction = player.direction;

        let player_entity =
            unwrap_or_log!(game.entities.iter_mut().find(|e| e.id == player.entity_id),
                           "Player entity disappeared");

        if player.left_click {
            player_entity.attack = true;
        }

        casts = update_player(player_entity, player);
    }

    if casts[0] {
        game.player.ability_0(&mut game.entities);
    }
    if casts[1] {
        game.player.ability_1(&mut game.entities);
    }
    if casts[2] {
        game.player.ability_2(&mut game.entities);
    }
    if casts[3] {
        game.player.ability_3(&mut game.entities);
    }
}

// Updates player related things that aren't TPS bound
pub fn update_player_non_tps_bound(game: &mut GameState,
                                   graphics: &mut GraphicsState,
                                   window: &Window) {

    graphics.update_camera(game.player.coords.clone(), game.player.direction);
    center_mouse(graphics, &mut game.player.mouse, window);
}
