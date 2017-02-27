//! Logic for the main menu
//!
//! Displays when the game is started, after it has finished loading
//! Shows buttons for navigating to the new game and options menu, and for exiting the game

use glutin::{Window, CursorState};
use conrod::{self, Widget, Colorable, Positionable, Sizeable, Labelable, widget, color};

use gui::ids::Ids;
use gui::UIState;
use gamestate::GameState;
use hsgraphics::GraphicsState;
use gameloop::LoopType;

/// Sets the widgets for the main menu
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

    // Opens the new game menu
    let new_game_button = widget::Button::new()
        .label("New game")
        .label_font_size(30)
        .label_color(color::BLUE)
        .w_h(225.0, 50.0)
        .color(color::TRANSPARENT)
        .up_from(ids.button_options, 0.0)
        .depth(-2.0)
        .set(ids.button_new_game, ui);

    // Opens the options menu
    let options_button = widget::Button::new()
        .label("Options")
        .label_font_size(30)
        .label_color(color::BLUE)
        .w_h(225.0, 50.0)
        .color(color::TRANSPARENT)
        .up_from(ids.button_quit, 0.0)
        .depth(-2.0)
        .set(ids.button_options, ui);

    // Closes the game
    let quit_button = widget::Button::new()
        .label("Quit")
        .label_font_size(30)
        .label_color(color::BLUE)
        .w_h(225.0, 50.0)
        .color(color::TRANSPARENT)
        .bottom_left_of(ids.background)
        .depth(-2.0)
        .set(ids.button_quit, ui);

    // Main menu background image
    widget::Image::new()
        .wh_of(ids.background)
        .middle()
        .depth(-1.0)
        .set(ids.main_menu_image, ui);

    if new_game_button.was_clicked() {
        *ui_state = UIState::NewGame;
    }

    if options_button.was_clicked() {
        *ui_state = UIState::Options;
    }

    if quit_button.was_clicked() {
        graphics.should_close = true;
    }
}
