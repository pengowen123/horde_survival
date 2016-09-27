use glutin::Window;

use gamestate::GameState;
use hsgraphics::GraphicsState;
use entity::update_player;
use hslog::CanUnwrap;
use hscontrols::center_mouse;

pub fn update_player_state(game: &mut GameState, graphics: &mut GraphicsState, window: &Window) {
    game.player.update_cooldowns();

    let casts;

    // Scoped for ability casts
    {
        let player = &mut game.player;
        let direction = player.direction;

        let player_entity = unwrap_or_log!(game.entities.iter_mut().find(|e| e.id == player.entity_id),
                                           "Player entity disappeared");

        if player.left_click {
            player_entity.attack = true;
        }

        graphics.update_camera(player.coords.clone(), direction);

        casts = update_player(player_entity,
                              &mut player.dead,
                              player.move_forward,
                              player.move_left,
                              player.move_right,
                              player.move_backward,
                              &player.mouse,
                              &mut player.direction,
                              &mut player.coords);
    }

    if casts[0] { game.player.ability_0(&mut game.entities); }
    if casts[1] { game.player.ability_1(&mut game.entities); }
    if casts[2] { game.player.ability_2(&mut game.entities); }
    if casts[3] { game.player.ability_3(&mut game.entities); }

    center_mouse(graphics, &mut game.player.mouse, window);
}
