use glutin::*;

use entity::*;
use gameloop::{self, LoopType};
use gamestate::GameState;
use hsgraphics::GraphicsState;
use utils::*;
use tps::Ticks;
use gui::{UI, UIState};

pub fn gametick(event: Option<Event>,
                ui: &mut UI,
                game: &mut GameState,
                graphics: &mut GraphicsState,
                window: &Window,
                ticks: &mut Ticks,
                loop_type: &mut LoopType) {

    ticks.begin_tick();

    if let Some(e) = event {
        if let Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) = e {
            *loop_type = LoopType::GUI;
            ui.state = UIState::EscapeMenu;
            set_cursor_state(window, CursorState::Normal);
            game.player.reset_controls();
            return;
        }

        gameloop::handle_event(e, game, graphics, window);
    }

    graphics.draw(window);

    ticks.measure_frame_1();

    if ticks.is_sleeping() {
        return;
    }

    gameloop::update_player_state(game, graphics, window);

    for i in 0..game.entities.len() {
        update_entity(&mut game.entities, i, &game.map, &mut game.player, &mut game.next_entity_id);
    }

    //update_clumped_entities(&mut game.entities);
    filter_entities(&mut game.entities);

    if game.round_finished() {
        *loop_type = LoopType::GUI;
        *ticks = Ticks::new();
        game.end_round(graphics);

        set_cursor_state(window, CursorState::Normal);
    }

    ticks.measure_update();

    graphics.update(game);

    ticks.measure_frame_2();
    ticks.end_tick();

    if graphics.options.display_debug {
        let info = ticks.get_debug_info();

        let frame = millis(info[0]);
        let update = millis(info[1]);
        let total = millis(info[2]);

        let string = format!("Horde Survival - frame {} ms | updates {} ms | total {} ms", frame, update, total);

        window.set_title(&string);
    }
}
