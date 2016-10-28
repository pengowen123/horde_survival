use glutin::{Window, CursorState};
use conrod::{
    self,
    Widget,
    Colorable,
    Positionable,
    Sizeable,
    Labelable,
    widget,
    color,
};

use gui::ids::Ids;
use gui::UIState;
use gamestate::GameState;
use hsgraphics::GraphicsState;
use gameloop::LoopType;
use utils::set_cursor_state;

pub fn set_widgets(ui: &mut conrod::UiCell,
                   ids: &Ids,
                   game: &mut GameState,
                   graphics: &mut GraphicsState,
                   ui_state: &mut UIState,
                   loop_type: &mut LoopType,
                   window: &Window) {

    let bg_color = ui.theme.background_color;

    // Fullscreen canvas
    widget::Canvas::new()
        .color(bg_color)
        .set(ids.background, ui);

    // Start new game
    // TODO: Send to New Game menu instead to adjust settings, and start game from there
    if widget::Button::new()
        .label("New game")
        .label_font_size(30)
        .label_color(color::BLUE)
        .w_h(225.0, 50.0)
        .color(color::TRANSPARENT)
        .up_from(ids.button_options, 0.0)
        .depth(-2.0)
        .set(ids.button_new_game, ui)
        .was_clicked() {
            game.new_game();
            game.next_round();
            set_cursor_state(window, CursorState::Hide);
            graphics.reset_cursor(window);
            *loop_type = LoopType::Game;
        }

    // Go to options menu
    if widget::Button::new()
        .label("Options")
        .label_font_size(30)
        .label_color(color::BLUE)
        .w_h(225.0, 50.0)
        .color(color::TRANSPARENT)
        .up_from(ids.button_quit, 0.0)
        .depth(-2.0)
        .set(ids.button_options, ui)
        .was_clicked() {
            *ui_state = UIState::Options;
        }

    // Exit game
    if widget::Button::new()
        .label("Quit")
        .label_font_size(30)
        .label_color(color::BLUE)
        .w_h(225.0, 50.0)
        .color(color::TRANSPARENT)
        .bottom_left_of(ids.background)
        .depth(-2.0)
        .set(ids.button_quit, ui)
        .was_clicked() {
            graphics.should_close = true;
        }

    widget::Image::new()
        .wh_of(ids.background)
        .middle()
        .depth(-1.0)
        .set(ids.main_menu_image, ui);
}
