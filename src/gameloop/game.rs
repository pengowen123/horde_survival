use glutin::*;

use consts::misc::TPS;
use gameloop::{self, LoopType};
use gamestate::GameState;
use hsgraphics::GraphicsState;
use tps::{Ticks, tps_to_time};
use gui::{UI, UIState};
use entity::*;
use utils::*;

pub fn gametick(events: Vec<Event>,
                ui: &mut UI,
                game: &mut GameState,
                graphics: &mut GraphicsState,
                window: &Window,
                ticks: &mut Ticks,
                loop_type: &mut LoopType) {

    // Set max TPS of game
    ticks.begin_tick();
    let expected_elapsed = tps_to_time(TPS);
    ticks.set_expected_elapsed(expected_elapsed);

    let mut pressed_escape = false;

    let mut last_mouse_move = None;
    let mut other_events = Vec::new();

    // Filters mouse events out, and gets the most recent one
    for event in events {
        if is_mouse_moved_event(&event) {
            last_mouse_move = Some(event);
        } else {
            // Detect whether Escape was pressed
            if let Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) =
                event {
                pressed_escape = true;
            }
            other_events.push(event);
        }
    }

    // Only process the last mouse movement
    if let Some(e) = last_mouse_move {
        other_events.push(e.clone());
    }

    if pressed_escape {
        *loop_type = LoopType::GUI;
        ui.state = UIState::Pause;
        set_cursor_state(window, CursorState::Normal);
        game.player.reset_controls();

        return;
    }

    for e in other_events {
        gameloop::handle_event(e, game, graphics, window);
    }

    gameloop::update_player_non_tps_bound(game, graphics, window);

    graphics.draw(window);

    ticks.measure_frame_1();

    if ticks.is_sleeping() {
        return;
    }

    gameloop::update_player_state(game, graphics);

    for i in 0..game.entities.len() {
        update_entity(&mut game.entities,
                      i,
                      &game.map,
                      &mut game.player,
                      &mut game.next_entity_id);
    }

    //::entity::update_clumped_entities(&mut game.entities);
    filter_entities(&mut game.entities);

    if game.round_finished() {
        *loop_type = LoopType::GUI;
        *ticks = Ticks::new();
        ui.state = UIState::Shop;
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

        let string = format!("Horde Survival - frame {} ms | updates {} ms | total {} ms",
                             frame,
                             update,
                             total);

        window.set_title(&string);
    }
}
