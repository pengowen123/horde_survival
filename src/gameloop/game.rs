//! Gameloop for LoopType::Game

use glutin::*;

use consts::misc::TPS;
use gameloop::{self, LoopType};
use gamestate::GameState;
use hsgraphics::GraphicsState;
use tps::{Ticks, tps_to_time};
use gui::{UI, UIState};
use entity::*;
use utils::*;

/// Runs the game in Game mode, meaning entities receive updates, the player can control
/// their character, and the camera's view is displayed
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

    // Whether Escape key was pressed
    let mut pressed_escape = false;
    let mut last_mouse_move = None;
    let mut other_events = Vec::new();

    // Filter outdated mouse positions
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

    // Only process the most recent mouse position
    if let Some(e) = last_mouse_move {
        other_events.push(e.clone());
    }

    // If the Escape key was pressed while in Game mode:
    // Set the mode to GUI
    // Set the GUI state to the pause screen
    // Reset the TPS tracker
    // Unhide the cursor
    // Reset the player controls
    // End the iteration early
    if pressed_escape {
        *loop_type = LoopType::GUI;
        ui.state = UIState::Pause;
        ticks.reset();
        set_cursor_state(window, CursorState::Normal);
        game.player.reset_controls();
        return;
    }

    // Handle events
    for e in other_events {
        gameloop::handle_event_game(e, game, graphics, window);
    }

    // Run non-TPS bound player updates
    gameloop::update_player_non_tps_bound(game, graphics, window);

    // Display the game
    graphics.draw_game(window);

    // Measure elapsed time
    ticks.measure_frame_1();

    // End the iteration if the game doesn't require an update yet
    // Updates after this point are bound by TPS
    if ticks.is_sleeping() {
        return;
    }

    // Run TPS bound player updates
    gameloop::update_player_state(game);

    // Update entities
    for i in 0..game.entities.len() {
        update_entity(&mut game.entities,
                      i,
                      &game.map,
                      &mut game.player,
                      &mut game.next_entity_id);
    }

    // FIXME: Uncomment this when the function is fixed (see the function's comment for more info)
    // :entity::update_clumped_entities(&mut game.entities);

    // Filter out entities that should be removed (such as dead ones)
    filter_entities(&mut game.entities);

    // If the round has been completed:
    // Set the mode to GUI
    // Set the GUI state to the Shop screen
    // Reset the TPS tracker
    // Unhide the cursor
    // Run the end of round function
    if game.is_round_finished() {
        *loop_type = LoopType::GUI;
        ticks.reset();
        ui.state = UIState::Shop;
        game.end_round(graphics);
        set_cursor_state(window, CursorState::Normal);
    }

    // Measure elapsed time
    ticks.measure_update();

    // Update the graphics state
    graphics.update(game);

    // Measure elapsed time
    ticks.measure_frame_2();

    // Display debug info if it is requested
    if graphics.options.display_debug {
        let info = ticks.get_debug_info();

        // Convert times to milliseconds
        let frame = millis(info[0]);
        let update = millis(info[1]);
        let total = millis(info[2]);

        // Format the info
        let string = format!("Horde Survival - frame {} ms | updates {} ms | total {} ms",
                             frame,
                             update,
                             total);

        // Display the info
        window.set_title(&string);
    }

    // Finish measurements
    ticks.end_tick();
}
