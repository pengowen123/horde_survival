//! Implementation of the main menu

use common::conrod::{self, Colorable, Positionable, color};
use common::conrod::widget::{self, Widget};
use common::{UiState, glutin};

use menus::{Menus, options};
use consts::{self, UI_BACKGROUND_COLOR, GENERIC_BUTTON_SPACING};

const TITLE_TEXT_FONT_SIZE: u32 = 46;

impl Menus {
    pub fn set_widgets_main_menu(
        &mut self,
        ui: &mut conrod::UiCell,
        ui_state: &mut UiState,
        window: &glutin::GlWindow,
    ) {
        let ids = &self.ids;

        // The main canvas
        widget::Canvas::new()
            .color(UI_BACKGROUND_COLOR)
            .set(ids.main_canvas, ui);

        // Title text
        widget::Text::new("Horde Survival")
            .mid_top_with_margin_on(ids.main_canvas, 75.0)
            .color(color::BLACK)
            .font_size(TITLE_TEXT_FONT_SIZE)
            .set(ids.title_text, ui);

        // Start game button
        if consts::create_generic_button(widget::Button::new(), "Start Game")
            .align_middle_x_of(ids.main_canvas)
            .align_middle_y_of(ids.main_canvas)
            .set(ids.start_game_button, ui)
            .was_clicked()
        {
            window.hide_cursor(true);
            self.set_ui_state(ui_state, UiState::InGame);
        }

        // Options menu button
        if consts::create_generic_button(widget::Button::new(), "Options")
            .y_relative(GENERIC_BUTTON_SPACING)
            .set(ids.main_menu_options_button, ui)
            .was_clicked()
        {
            self.set_ui_state(ui_state, UiState::OptionsMenu);
            self.options_menu_return_to = Some(options::ReturnTo::MainMenu);
        }

        // Exit button
        if consts::create_generic_button(widget::Button::new(), "Exit Game")
            .y_relative(GENERIC_BUTTON_SPACING)
            .set(ids.exit_button, ui)
            .was_clicked()
        {
            self.set_ui_state(ui_state, UiState::Exit);
        }
    }
}
