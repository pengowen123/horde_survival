//! Logic for the new game menu
//!
//! Displays when the player is selecting settings for starting a new game
//! Shows TODO

use glutin::{Window, CursorState};
use conrod::{self, Widget, Colorable, Positionable, Sizeable, Labelable, widget, color};

use gui::ids::Ids;
use gui::UIState;
use gamestate::GameState;
use hsgraphics::GraphicsState;
use gameloop::LoopType;
use utils::set_cursor_state;

/// Sets the widgets for the new game menu
pub fn set_widgets(ui: &mut conrod::UiCell,
                   ids: &Ids,
                   game: &mut GameState,
                   graphics: &mut GraphicsState,
                   ui_state: &mut UIState,
                   loop_type: &mut LoopType,
                   window: &Window) {

    // Get the background color
    let bg_color = ui.theme.background_color;

    // Fullscreen canvas
    widget::Canvas::new()
        .color(bg_color)
        .set(ids.background, ui);

    // Starts the game
    let start_game_button = widget::Button::new()
        .label("Start game")
        .label_font_size(30)
        .label_color(color::BLUE)
        .w_h(225.0, 50.0)
        .color(color::TRANSPARENT)
        .bottom_right_with_margin_on(ids.background, 50.0)
        .depth(-2.0)
        .set(ids.button_new_game, ui);

    if start_game_button.was_clicked() {
        game.new_game();
        game.next_round();
        set_cursor_state(window, CursorState::Hide);
        graphics.reset_cursor(window);
        *loop_type = LoopType::Game;
    }
}
