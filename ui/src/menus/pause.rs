//! Implementation of the pause menu

use common::conrod::widget::{self, Widget};
use common::conrod::{self, Colorable, Positionable};
use common::{glutin, config, UiState};
use window::window_event;

use consts::{self, GENERIC_BUTTON_SPACING, UI_BACKGROUND_COLOR};
use menus::{options, Menus};

impl Menus {
    pub fn set_widgets_pause_menu(
        &mut self,
        ui: &mut conrod::UiCell,
        ui_state: &mut UiState,
        window: &glutin::GlWindow,
        event_channel: &mut window_event::EventChannel,
        config: &mut config::Config,
    ) {
        let ids = &self.ids;

        // Root canvas
        widget::Canvas::new().set(ids.pause_menu_root_canvas, ui);

        // The main canvas
        widget::Canvas::new()
            .color(UI_BACKGROUND_COLOR)
            .middle_of(ids.pause_menu_root_canvas)
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

        // Options menu button
        if consts::create_generic_button(widget::Button::new(), "Options")
            .y_relative(GENERIC_BUTTON_SPACING)
            .set(ids.pause_menu_options_button, ui)
            .was_clicked()
        {
            self.set_ui_state(ui_state, UiState::OptionsMenu);
            self.options_menu_return_to = Some(options::ReturnTo::PauseMenu);
        }

        // Exit to main menu button
        if consts::create_generic_button(widget::Button::new(), "Exit to Main Menu")
            .y_relative(GENERIC_BUTTON_SPACING)
            .set(ids.exit_to_main_menu_button, ui)
            .was_clicked()
        {
            self.set_ui_state(ui_state, UiState::MainMenu);
        }

        // Auto-revert window settings pop-up
        let redraw = if self.showing_auto_revert() {
            options::auto_revert_popup(
                &mut self.auto_revert_state,
                &mut self.current_config,
                &mut self.new_config,
                config,
                ids,
                ids.pause_menu_root_canvas,
                ui,
                event_channel,
            )
        } else {
            false
        };

        if redraw {
            self.set_force_redraw(true);
        }
    }
}
