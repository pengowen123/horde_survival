//! Logic for the shop menu
//!
//! Displays at the end of each round so the player can purchase items
//! Shows TODO

use glutin::{Window, CursorState};
use conrod::{self, Widget, Colorable, Positionable, Sizeable, Labelable, Borderable, widget, color};
use conrod::widget::id::Id;

use consts::graphics::gui::SHOP_MATRIX_COLUMNS;
use gui::ids::Ids;
use gui::UIState;
use gui::state::{WidgetStates, ShopMatrix};
use gamestate::GameState;
use hsgraphics::GraphicsState;
use gameloop::LoopType;
use utils::set_cursor_state;

/// Sets the widgets for the shop menu
#[allow(too_many_arguments)]
pub fn set_widgets(ui: &mut conrod::UiCell,
                   ids: &Ids,
                   game: &mut GameState,
                   graphics: &mut GraphicsState,
                   ui_state: &mut UIState,
                   widget_states: &mut WidgetStates,
                   loop_type: &mut LoopType,
                   window: &Window) {

    // Get the background color
    let bg_color = ui.theme.background_color;

    // Fullscreen canvas
    widget::Canvas::new()
        .color(bg_color)
        .set(ids.background, ui);

    // Starts the next round
    let next_round_button = widget::Button::new()
        .label("Next round")
        .label_font_size(30)
        .label_color(color::BLUE)
        .w_h(225.0, 50.0)
        .color(color::TRANSPARENT)
        .bottom_right_with_margins_on(ids.background, 25.0, 0.0)
        .depth(-2.0)
        .set(ids.button_next_round, ui);

    if next_round_button.was_clicked() {
        *loop_type = LoopType::Game;
        set_cursor_state(window, CursorState::Hide);
        graphics.reset_cursor(window);
        game.next_round();
    }

    // Canvas to hold the items matrix
    widget::Canvas::new()
        .scroll_kids_vertically()
        .w_h(400.0, 400.0)
        .mid_left_with_margin_on(ids.background, 125.0)
        .set(ids.canvas_items, ui);

    // Scrollbar for the items matrix canvas
    widget::Scrollbar::y_axis(ids.canvas_items)
        // Stops the scrollbar from overlapping with the item buttons
        .right(0.0)
        .thickness(15.0)
        .color(color::LIGHT_RED)
        .set(ids.scrollbar_items, ui);

    create_item_matrix(ui,
                       ids.canvas_items,
                       ids.matrix_items,
                       &mut widget_states.weapon_matrix)

    // TODO: CONTINUE HERE
    //       Use create_item_matrix to make item tabs
}

/// Creates and returns a matrix of buttons to select items from the shop with
fn create_item_matrix(ui: &mut conrod::UiCell,
                      canvas_id: Id,
                      matrix_id: Id,
                      items: &mut ShopMatrix) {

    // The dimensions of the matrix
    let columns = SHOP_MATRIX_COLUMNS;
    let rows = items.len();

    // A matrix of items that can be purchased
    let mut elements = widget::Matrix::new(columns, rows)
        .w_of(canvas_id)
        .h(100.0 * rows as conrod::Scalar)
        .crop_kids()
        .middle_of(canvas_id)
        .set(matrix_id, ui);

    while let Some(elem) = elements.next(ui) {
        let (col, row) = (elem.col, elem.row);

        let item = &mut items[row][col];

        // If the item is a dummy (used as an empty slot in the matrix), display a rectangle instead
        if !item.1.is_dummy() {
            let (state, item) = (&mut item.0, &mut item.1);

            let button = widget::Button::new()
                .label("foo")
                .label_color(color::BLACK)
                .color(color::BLUE)
                .border(7.5)
                .border_color(color::DARK_BLUE);

            elem.set(button, ui);
        } else {
            let rectangle = widget::Rectangle::fill([0.0; 2]).color(color::DARK_BLUE);
            elem.set(rectangle, ui);
        }
    }
}
