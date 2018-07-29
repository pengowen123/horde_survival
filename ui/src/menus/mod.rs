//! Implementation of game menus

mod main;
mod ingame;
mod pause;
mod options;

use std::sync::atomic::{AtomicBool, Ordering};

use common::conrod::widget::id;
use common::config;
use UiState;

widget_ids! {
    pub struct Ids {
        // Main menu
        main_canvas,
        title_text,
        exit_button,
        start_game_button,
        main_menu_options_button,
        // Pause menu
        pause_canvas,
        resume_game_button,
        exit_to_main_menu_button,
        pause_menu_options_button,
        // Options menu
        options_root_canvas,
        options_top_canvas,
        options_bottom_canvas,
        options_submenu_tabs,
        options_graphics_canvas,
        options_window_canvas,
        options_camera_canvas,
        options_bindings_canvas,
        reset_graphics_button,
        reset_window_button,
        reset_camera_button,
        reset_binds_button,
        apply_button,
        back_button,
        cancel_button,
        fov_canvas,
        fov_label,
        fov_slider,
        sensitivity_canvas,
        sensitivity_label,
        sensitivity_slider,
    }
}

/// Stores the state required to run each menu
pub struct Menus {
    ids: Ids,
    ui_state_changed: AtomicBool,
    options_menu_return_to: Option<options::ReturnTo>,
    /// The current config (the current value of the `Config` resource, converted to
    /// `ConfigUiState`)
    current_config: options::ConfigUiState,
    /// The config that is edited in the options menu. Upon pressing the `Apply` button, the value
    /// of `new_config` is written to both `current_config` and the `Config` resource
    new_config: options::ConfigUiState,
}

impl Menus {
    pub fn new(config: config::Config, ui: id::Generator) -> Self {
        let config: options::ConfigUiState = config.into();
        Menus {
            ids: Ids::new(ui),
            ui_state_changed: false.into(),
            options_menu_return_to: None,
            current_config: config.clone(),
            new_config: config,
        }
    }

    /// Returns whether the UI state was updated by a menu
    ///
    /// Also resets the flag that tracks this information.
    pub fn did_ui_state_change(&mut self) -> bool {
        let result = self.ui_state_changed.load(Ordering::SeqCst);
        self.ui_state_changed.store(false, Ordering::SeqCst);
        result
    }

    /// Sets `ui_state` to `new_state`, and sets the `ui_state_changed` flag to `true`
    ///
    /// This method is used to prevent accidentally forgetting to set `ui_state_changed` when
    /// changing UI state.
    fn set_ui_state(&self, ui_state: &mut UiState, new_state: UiState) {
        self.ui_state_changed.store(true, Ordering::SeqCst);
        *ui_state = new_state;
    }
}
