//! Implementation of game menus

mod main;
mod ingame;
mod pause;
mod options;

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use common::conrod::widget::id;
use common::config;
use UiState;

widget_ids! {
    pub struct Ids {
        // Main menu
        main_menu_root_canvas,
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
        auto_revert_big_canvas,
        auto_revert_canvas,
        keep_changes_canvas,
        keep_changes_text,
        auto_revert_text,
        keep_changes_button,
        revert_changes_button,
        changes_require_restart_text,
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
        window_size_canvas,
        window_size_label,
        window_size_button_left,
        window_size_button_right,
        window_size_text_canvas,
        window_size_text,
        fullscreen_canvas,
        fullscreen_label,
        fullscreen_button,
        vsync_canvas,
        vsync_label,
        vsync_button,
    }
}

pub struct AutoRevertState {
    /// The previous window dimensions, to be reverted to if the "revert changes" button is pressed
    old_dimensions: options::WindowDimensions,
    /// The previous fullscreen setting
    old_fullscreen: bool,
    popup_start_time: Instant,
}

impl AutoRevertState {
    fn new(old_dimensions: options::WindowDimensions, old_fullscreen: bool) -> Self {
        Self {
            old_dimensions,
            old_fullscreen,
            popup_start_time: Instant::now(),
        }
    }
}

/// Stores the state required to run each menu
pub struct Menus {
    ids: Ids,
    force_redraw: AtomicBool,
    options_menu_return_to: Option<options::ReturnTo>,
    /// State for the auto-revert window settings pop-up
    ///
    /// This field is `Some(..)` when the pop-up is currently being shown
    auto_revert_state: Option<AutoRevertState>,
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
            force_redraw: false.into(),
            options_menu_return_to: None,
            auto_revert_state: None,
            current_config: config.clone(),
            new_config: config,
        }
    }

    /// Returns whether the auto-revert windows settings pop-up is currently showing
    pub fn showing_auto_revert(&self) -> bool {
        self.auto_revert_state.is_some()
    }

    /// Returns whether the UI should be forced to be redrawn
    pub fn should_force_redraw(&self) -> bool {
        let result = self.force_redraw.load(Ordering::SeqCst);
        result
    }

    /// Sets the `force_redraw` flag to the provided value
    pub fn set_force_redraw(&self, value: bool) {
        self.force_redraw.store(value, Ordering::SeqCst);
    }

    /// Sets `ui_state` to `new_state`, and sets the `force_redraw` flag to `true`
    ///
    /// This method is used to prevent accidentally forgetting to set `force_redraw` when
    /// changing UI state.
    fn set_ui_state(&self, ui_state: &mut UiState, new_state: UiState) {
        self.set_force_redraw(true);
        *ui_state = new_state;
    }
}
