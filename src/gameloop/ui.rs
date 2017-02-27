use glutin::*;
use conrod;

use gamestate::GameState;
use hsgraphics::GraphicsState;
use gameloop::{LoopType, handle_event_gui};
use gui::{UI, draw};
use tps::{Ticks, tps_to_time};
use consts::misc::GUI_MAX_FPS;
use utils::*;

/// Runs the game in GUI mode, meaning the GUI is displayed, and the user can interact with it
pub fn run_gui(events: Vec<Event>,
               ui: &mut UI,
               game: &mut GameState,
               graphics: &mut GraphicsState,
               window: &Window,
               ticks: &mut Ticks,
               loop_type: &mut LoopType) {

    // Set max fps of gui
    ticks.begin_tick();
    let expected_elapsed = tps_to_time(GUI_MAX_FPS);
    ticks.set_expected_elapsed(expected_elapsed);

    // Get size of window and update dpi
    let (w, h) = if let Some(s) = window.get_inner_size() {
        s
    } else {
        graphics.should_close = true;
        return;
    };
    graphics.update_dpi(window);

    // Tell conrod to update the ui
    let dt_secs = 0.0;
    ui.ui.handle_event(conrod::event::render(dt_secs, w, h, graphics.dpi as conrod::Scalar));

    let mut last_mouse_move = None;
    let mut other_events = Vec::new();

    // Filter outdated mouse positions
    for event in events {
        if is_mouse_moved_event(&event) {
            last_mouse_move = Some(event)
        } else {
            other_events.push(event);
        }
    }

    // Only process the most recent mouse position
    if let Some(e) = last_mouse_move {
        other_events.push(e.clone());
    }

    // Handle glutin events
    for e in other_events {
        let (w, h) = (w as conrod::Scalar, h as conrod::Scalar);
        let dpi = graphics.dpi as conrod::Scalar;

        // Give the converted event to conrod
        if let Some(event) = conrod::backend::glutin::convert(e.clone(), w, h, dpi) {
            ui.ui.handle_event(event);
        }

        // Handle event
        handle_event_gui(e, ui, graphics, window, loop_type);
    }

    // Don't draw ui if the tick hasn't finished yet
    if ticks.is_sleeping() {
        return;
    }

    // Get the background color
    let bg_color = ui.ui.theme.background_color.to_fsa();

    // Draw primitives generated by conrod, but only if the ui has changed
    if let Some(primitives) = ui.ui.draw_if_changed() {
        // Clear the screen
        graphics.encoder.clear(&graphics.data3d.out_color, bg_color);

        // Draw primitives (doesn't actually display them)
        draw::draw_primitives(primitives, (w, h), graphics, &ui.image_map);

        // Display the GUI
        graphics.draw_gui(window);
    }

    // Make the list of widgets to put in the GUI
    ui.set_widgets(game, graphics, loop_type, window);

    // Measure elapased time
    ticks.measure_frame_1();

    // Display debug info
    if graphics.options.display_debug {
        // Convert times to milliseconds
        let frame = millis(ticks.get_debug_info()[0]);

        // Format the info
        let string = format!("Horde Survival - frame {} ms | updates 0 ms | total {} ms",
                             frame,
                             frame);

        // Display the info
        window.set_title(&string);
    }

    // Finish measurements
    ticks.end_tick();
}
