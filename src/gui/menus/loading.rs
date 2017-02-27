//! Logic for the loading menu
//!
//! Displays at each loading screen, such as when the game starts
//! Shows TODO

use glutin::Window;
use conrod::{self, Widget, Colorable, Positionable, Sizeable, Labelable, widget, color};

use gui::ids::Ids;
use gui::UIState;
use gamestate::GameState;
use hsgraphics::GraphicsState;
use gameloop::LoopType;

/// Sets the widgets for the loading menu
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
}
