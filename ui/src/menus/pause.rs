//! Implementation of the pause menu

use common::conrod::{self, Colorable, Positionable};
use common::conrod::widget::{self, Widget};
use common::{UiState, glutin};
use window::window_event;

use menus::Menus;
use consts::{self, UI_BACKGROUND_COLOR, GENERIC_BUTTON_SPACING};

impl Menus {
    pub fn set_widgets_pause_menu(
        &mut self,
        ui: &mut conrod::UiCell,
        ui_state: &mut UiState,
        window: &glutin::GlWindow,
        event_channel: &mut window_event::EventChannel,
    ) {
        let ids = &self.ids;

        // The main canvas
        widget::Canvas::new()
            .color(UI_BACKGROUND_COLOR)
            .set(ids.pause_canvas, ui);

        // Resume game button
        if consts::create_generic_button(widget::Button::new(), "Resume Game")
            .align_middle_x_of(ids.pause_canvas)
            .align_middle_y_of(ids.pause_canvas)
            .set(ids.resume_game_button, ui)
            .was_clicked()
        {
            window_event::unpause(ui_state, window, event_channel);
        }

        // Exit to main menu button
        if consts::create_generic_button(widget::Button::new(), "Exit to Main Menu")
            .y_relative(GENERIC_BUTTON_SPACING)
            .set(ids.exit_to_main_menu_button, ui)
            .was_clicked()
        {
            *ui_state = UiState::MainMenu;
        }
    }
}
