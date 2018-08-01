//! Implementation of the options menu

use common::{UiState, gfx, config, glutin};
use common::conrod::{self, Colorable, Positionable, Sizeable, Labelable, color};
use common::conrod::widget::{self, Widget};
use window::window_event;
use slog;
use petgraph;

use std::{fmt, cmp};

use menus::{Menus, Ids, AutoRevertState, WaitForKeypressState};
use consts::{self, UI_BACKGROUND_COLOR};
use theme;

const AUTO_REVERT_TIME: u64 = 15;
const OPTIONS_TRANSITION_BUTTON_WIDTH: conrod::Scalar = 200.0;
const OPTIONS_TRANSITION_SPACING: conrod::Scalar = OPTIONS_TRANSITION_BUTTON_WIDTH * 1.25;
const OPTION_NAME_FONT_SIZE: u32 = 26;
const OPTION_NAME_COLOR: color::Color = color::BLACK;
const OPTION_LABEL_FONT_SIZE: u32 = 18;
const OPTION_HEIGHT: conrod::Scalar = 50.0;
const OPTION_MARGIN: conrod::Scalar = 25.0;
const OPTION_CANVAS_COLOR: color::Color = color::LIGHT_ORANGE;
const OPTION_SLIDER_WIDTH: conrod::Scalar = 200.0;

/// A version of `Config` that is suitable for editing by the options menu
#[derive(Clone, PartialEq)]
pub struct ConfigUiState {
    pub graphics: GraphicsConfig,
    pub window: WindowConfig,
    pub camera: config::CameraConfig,
    pub bindings: config::BindConfig,
}

impl Into<config::Config> for ConfigUiState {
    fn into(self) -> config::Config {
        let mut camera = self.camera;

        // Scale sensitivity so that 1.0 is DEFAULT_SENSITIVITY
        camera.sensitivity *= config::DEFAULT_SENSITIVITY;
        
        config::Config {
            graphics: self.graphics.into(),
            window: self.window.into(),
            camera: camera,
            bindings: self.bindings,
        }
    }
}

impl From<config::Config> for ConfigUiState {
    fn from(config: config::Config) -> Self {
        let mut camera = config.camera;

        camera.sensitivity /= config::DEFAULT_SENSITIVITY;

        Self {
            graphics: config.graphics.into(),
            window: config.window.into(),
            camera,
            bindings: config.bindings,
        }
    }
}

/// A trait for options that have variants that can be selected with left/right buttons, such as
/// for the window size option
trait SelectOption {
    /// Returns the next option after this one
    ///
    /// Returns `self` if it is at or after the provided variant
    fn next(&self, max: Self) -> Self;
    /// Returns the previous option from this one
    ///
    /// Returns `self` if it is already the first variant
    fn previous(&self) -> Self;
}

#[derive(Clone, PartialEq)]
pub enum ShadowMapSize {
    _512,
    _1024,
    _2048,
    Custom(gfx::texture::Size),
}

impl ShadowMapSize {
    /// Returns an `i32` representing the index of this shadow map size
    ///
    /// Returns `0` for the minimum shadow map size, and `n` for the maximum shadow map size, where
    /// `n` is the number of shadow map size variants there are excluding `Custom`.
    ///
    /// Returns `-1` for `Custom`.
    fn to_i32(&self) -> i32 {
        match *self {
            ShadowMapSize::_512 => 0,
            ShadowMapSize::_1024 => 1,
            ShadowMapSize::_2048 => 2,
            ShadowMapSize::Custom(_) => -1,
        }
    }
}
impl SelectOption for ShadowMapSize {
    fn next(&self, max: Self) -> Self {
        if let ShadowMapSize::Custom(_) = *self {
            return ShadowMapSize::_512;
        } else if self.to_i32() >= max.to_i32() {
            return self.clone();
        }

        match *self {
            ShadowMapSize::_512 => ShadowMapSize::_1024,
            ShadowMapSize::_1024 => ShadowMapSize::_2048,
            ShadowMapSize::_2048 => ShadowMapSize::_2048,
            ShadowMapSize::Custom(_) => unreachable!(),
        }
    }

    fn previous(&self) -> Self {
        match *self {
            ShadowMapSize::_512 => ShadowMapSize::_512,
            ShadowMapSize::_1024 => ShadowMapSize::_512,
            ShadowMapSize::_2048 => ShadowMapSize::_1024,
            ShadowMapSize::Custom(_) => ShadowMapSize::_512,
        }
    }
}

impl fmt::Display for ShadowMapSize {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let size: gfx::texture::Size = self.clone().into();
        writeln!(fmt, "{}", size)
    }
}

impl Into<gfx::texture::Size> for ShadowMapSize {
    fn into(self) -> gfx::texture::Size {
        match self {
            ShadowMapSize::_512 => 512,
            ShadowMapSize::_1024 => 1024,
            ShadowMapSize::_2048 => 2048,
            ShadowMapSize::Custom(size) => size,
        }
    }
}

impl From<gfx::texture::Size> for ShadowMapSize {
    fn from(size: gfx::texture::Size) -> Self {
        match size {
            512 => ShadowMapSize::_512,
            1024 => ShadowMapSize::_1024,
            2048 => ShadowMapSize::_2048,
            other => ShadowMapSize::Custom(other),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct GraphicsConfig {
    pub postprocessing: bool,
    pub shadows: bool,
    pub shadow_map_size: ShadowMapSize,
}

impl Into<config::GraphicsConfig> for GraphicsConfig {
    fn into(self) -> config::GraphicsConfig {
        config::GraphicsConfig {
            postprocessing: self.postprocessing,
            shadows: self.shadows,
            shadow_map_size: self.shadow_map_size.into(),
        }
    }
}

impl From<config::GraphicsConfig> for GraphicsConfig {
    fn from(config: config::GraphicsConfig) -> Self {
        Self {
            postprocessing: config.postprocessing,
            shadows: config.shadows,
            shadow_map_size: config.shadow_map_size.into(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum WindowDimensions {
    _800x600,
    _1024x768,
    _1440x1080,
    _1920x1080,
    Custom(u32, u32),
}

impl WindowDimensions {
    /// Returns a number representing this `WindowDimensions`
    ///
    /// Returns `0` for the minimum window size, and `n` for the maximum window size, where `n` is
    /// the number of window size variants excluding `Custom`.
    ///
    /// Returns `-1` for `Custom`.
    fn to_i32(&self) -> i32 {
        match *self {
            WindowDimensions::_800x600 => 0,
            WindowDimensions::_1024x768 => 1,
            WindowDimensions::_1440x1080 => 2,
            WindowDimensions::_1920x1080 => 3,
            WindowDimensions::Custom(..) => -1,
        }
    }
}

impl SelectOption for WindowDimensions {
    fn next(&self, max_window_size: WindowDimensions) -> WindowDimensions {
        if let WindowDimensions::Custom(..) = *self {
            // There isn't anything to do here that makes sense other than return a default value
            WindowDimensions::_800x600
        } else if self >= &max_window_size {
            self.clone()
        } else {
            match *self {
                WindowDimensions::_800x600 => WindowDimensions::_1024x768,
                WindowDimensions::_1024x768 => WindowDimensions::_1440x1080,
                WindowDimensions::_1440x1080 => WindowDimensions::_1920x1080,
                WindowDimensions::_1920x1080 => WindowDimensions::_1920x1080,
                WindowDimensions::Custom(..) => unreachable!(),
            }
        }
    }

    fn previous(&self) -> WindowDimensions {
        match *self {
            WindowDimensions::_800x600 => WindowDimensions::_800x600,
            WindowDimensions::_1024x768 => WindowDimensions::_800x600,
            WindowDimensions::_1440x1080 => WindowDimensions::_1024x768,
            WindowDimensions::_1920x1080 => WindowDimensions::_1440x1080,
            // There isn't anything to do here that makes sense other than return a default value
            WindowDimensions::Custom(..) => WindowDimensions::_800x600,
        }
    }
}

impl fmt::Display for WindowDimensions {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let (w, h) = self.clone().into();
        writeln!(fmt, "{}x{}", w, h)
    }
}

impl cmp::PartialOrd for WindowDimensions {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        if let WindowDimensions::Custom(..) = *self {
            None
        } else if let WindowDimensions::Custom(..) = *other {
            None
        } else {
            self.to_i32().partial_cmp(&other.to_i32())
        }
    }
}

impl Into<(u32, u32)> for WindowDimensions {
    fn into(self) -> (u32, u32) {
        match self {
            WindowDimensions::_800x600 => (800, 600),
            WindowDimensions::_1024x768 => (1024, 768),
            WindowDimensions::_1440x1080 => (1440, 1080),
            WindowDimensions::_1920x1080 => (1920, 1080),
            WindowDimensions::Custom(w, h) => (w, h),
        }
    }
}

impl From<(u32, u32)> for WindowDimensions {
    fn from(dims: (u32, u32)) -> Self {
        match dims {
            (800, 600) => WindowDimensions::_800x600,
            (1024, 768) => WindowDimensions::_1024x768,
            (1440, 1080) => WindowDimensions::_1440x1080,
            (1920, 1080) => WindowDimensions::_1920x1080,
            (w, h) => WindowDimensions::Custom(w, h),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct WindowConfig {
    pub dimensions: WindowDimensions,
    pub fullscreen: bool,
    pub vsync: bool,
}

impl Into<config::WindowConfig> for WindowConfig {
    fn into(self) -> config::WindowConfig {
        let (width, height): (u32, u32) = self.dimensions.into();
        config::WindowConfig {
            width,
            height,
            fullscreen: self.fullscreen,
            vsync: self.vsync,
        }
    }
}

impl From<config::WindowConfig> for WindowConfig {
    fn from(config: config::WindowConfig) -> Self {
        WindowConfig {
            dimensions: WindowDimensions::from((config.width, config.height)),
            fullscreen: config.fullscreen,
            vsync: config.vsync,
        }
    }
}

/// Represents which menu to return to upon exiting the options menu
pub enum ReturnTo {
    MainMenu,
    PauseMenu,
}

impl Into<UiState> for ReturnTo {
    fn into(self) -> UiState {
        match self {
            ReturnTo::MainMenu => UiState::MainMenu,
            ReturnTo::PauseMenu => UiState::PauseMenu,
        }
    }
}

impl Menus {
    pub fn set_widgets_options_menu(
        &mut self,
        ui: &mut conrod::UiCell,
        ui_state: &mut UiState,
        keypress: Option<glutin::KeyboardInput>,
        event_channel: &mut window_event::EventChannel,
        config: &mut config::Config,
        log: &slog::Logger,
    ) {
        let ids = &self.ids;

        let ui_height = ui.window_dim()[1];

        let top_canvas_height_pct = 0.85;
        let bottom_canvas_height_pct = 1.0 - top_canvas_height_pct;

        // Root canvas
        widget::Canvas::new()
            .set(ids.options_root_canvas, ui);

        // Top canvas
        widget::Canvas::new()
            .color(color::ORANGE)
            .h(ui_height * top_canvas_height_pct)
            .mid_top_of(ids.options_root_canvas)
            .set(ids.options_top_canvas, ui);

        // Bottom canvas
        widget::Canvas::new()
            .color(UI_BACKGROUND_COLOR)
            .h(ui_height * bottom_canvas_height_pct)
            .mid_bottom_of(ids.options_root_canvas)
            .set(ids.options_bottom_canvas, ui);

        // Options sub-menu tabs
        widget::Tabs::new(&[
            (ids.options_camera_canvas, "Camera"),
            (ids.options_window_canvas, "Window"),
            (ids.options_graphics_canvas, "Graphics"),
            (ids.options_bindings_canvas, "Key Bindings"),
            ])
            .middle_of(ids.options_top_canvas)
            .wh_of(ids.options_top_canvas)
            .bar_thickness(50.0)
            .set(ids.options_submenu_tabs, ui);

        let mut exit_options_menu = false;
        let mut update_config = false;

        // Back button
        if options_transition_button(widget::Button::new(), "Back")
            .middle_of(ids.options_bottom_canvas)
            .set(ids.back_button, ui)
            .was_clicked()
        {
            exit_options_menu = true;
            update_config = true;
        }

        // Apply button
        if options_transition_button(widget::Button::new(), "Apply")
            .x_relative_to(ids.back_button, -OPTIONS_TRANSITION_SPACING)
            .set(ids.apply_button, ui)
            .was_clicked()
        {
            update_config = true;
        }

        // Cancel button
        if options_transition_button(widget::Button::new(), "Cancel")
            .x_relative_to(ids.back_button, OPTIONS_TRANSITION_SPACING)
            .set(ids.cancel_button, ui)
            .was_clicked()
        {
            // Reset changes to new config
            self.new_config = self.current_config.clone();
            exit_options_menu = true;
        }

        let mut camera_option_index = 0;
        // FOV option
        option_canvas(
            &mut camera_option_index,
            ids.fov_canvas,
            ids.options_camera_canvas,
            ui,
        );

        option_label(
            "Field of view",
            ids.fov_label,
            ids.fov_canvas,
            ui,
        );

        if let Some(new_fov) = widget::Slider::new(self.new_config.camera.fov.round(), 30.0, 120.0)
            .mid_right_with_margin_on(ids.fov_canvas, OPTION_MARGIN)
            .label(&self.new_config.camera.fov.round().to_string())
            .label_font_size(OPTION_LABEL_FONT_SIZE)
            .w(OPTION_SLIDER_WIDTH)
            .set(ids.fov_slider, ui)
        {
            self.new_config.camera.fov = new_fov.round();
        }

        // Sensitivity option
        option_canvas(
            &mut camera_option_index,
            ids.sensitivity_canvas,
            ids.options_camera_canvas,
            ui,
        );

        option_label(
            "Sensitivity",
            ids.sensitivity_label,
            ids.sensitivity_canvas,
            ui,
        );

        if let Some(new_sensitivity) =
            widget::Slider::new(self.new_config.camera.sensitivity, 0.1, 4.0)
                .mid_right_with_margin_on(ids.sensitivity_canvas, OPTION_MARGIN)
                .label(&format!("{:.*}", 2, self.new_config.camera.sensitivity))
                .label_font_size(OPTION_LABEL_FONT_SIZE)
                .w(OPTION_SLIDER_WIDTH)
                .set(ids.sensitivity_slider, ui)
        {
            self.new_config.camera.sensitivity = new_sensitivity;
        }

        let mut window_option_index = 0;
        // Window size option
        option_canvas(
            &mut window_option_index,
            ids.window_size_canvas,
            ids.options_window_canvas,
            ui,
        );

        option_label(
            "Window Size",
            ids.window_size_label,
            ids.window_size_canvas,
            ui,
        );

        if option_selector(
            &mut self.new_config.window.dimensions,
            // TODO: Don't let the window dimensions be set greater than the monitor size by
            // calculating a `WindowDimensions` from the monitor size
            WindowDimensions::_1920x1080,
            ids.window_size_button_left,
            ids.window_size_button_right,
            ids.window_size_text_canvas,
            ids.window_size_text,
            ids.window_size_canvas,
            ui,
        ) {
            self.set_force_redraw(true);
        };

        // Fullscreen option
        option_canvas(
            &mut window_option_index,
            ids.fullscreen_canvas,
            ids.options_window_canvas,
            ui,
        );

        option_label(
            "Fullscreen",
            ids.fullscreen_label,
            ids.fullscreen_canvas,
            ui,
        );

        if toggle_button(
            &mut self.new_config.window.fullscreen,
            ids.fullscreen_button,
            ids.fullscreen_canvas,
            // Extra margin to line up with the window size selector
            100.0,
            ui,
        ) {
            self.set_force_redraw(true);
        }

        // V-sync option
        option_canvas(
            &mut window_option_index,
            ids.vsync_canvas,
            ids.options_window_canvas,
            ui,
        );

        option_label(
            "V-sync",
            ids.vsync_label,
            ids.vsync_canvas,
            ui,
        );

        if toggle_button(
            &mut self.new_config.window.vsync,
            ids.vsync_button,
            ids.vsync_canvas,
            // Extra margin to line up with the window size selector
            100.0,
            ui,
        ) {
            self.set_force_redraw(true);
        }

        // Warn about v-sync changes requiring restart
        if self.new_config.window.vsync != self.current_config.window.vsync {
            warning_text(
                "Changes require restart",
                theme::FONT_COLOR,
                ids.changes_require_restart_text,
                ids.options_window_canvas,
                ui,
            );
        }

        let mut graphics_option_index = 0;
        // Postprocessing option
        option_canvas(
            &mut graphics_option_index,
            ids.postprocessing_canvas,
            ids.options_graphics_canvas,
            ui,
        );

        option_label(
            "Postprocessing",
            ids.postprocessing_label,
            ids.postprocessing_canvas,
            ui,
        );

        if toggle_button(
            &mut self.new_config.graphics.postprocessing,
            ids.postprocessing_button,
            ids.postprocessing_canvas,
            // Extra margin to line up with the shadow map size selector
            100.0,
            ui,
        ) {
            self.set_force_redraw(true);
        }

        // Shadows option
        option_canvas(
            &mut graphics_option_index,
            ids.shadows_canvas,
            ids.options_graphics_canvas,
            ui,
        );

        option_label(
            "Shadows",
            ids.shadows_label,
            ids.shadows_canvas,
            ui,
        );

        if toggle_button(
            &mut self.new_config.graphics.shadows,
            ids.shadows_button,
            ids.shadows_canvas,
            // Extra margin to line up with the shadow map size selector
            100.0,
            ui,
        ) {
            self.set_force_redraw(true);
        }

        // Shadow map size option
        option_canvas(
            &mut graphics_option_index,
            ids.shadow_map_size_canvas,
            ids.options_graphics_canvas,
            ui,
        );

        option_label(
            "Shadow Map Size",
            ids.shadow_map_size_label,
            ids.shadow_map_size_canvas,
            ui,
        );

        if option_selector(
            &mut self.new_config.graphics.shadow_map_size,
            ShadowMapSize::_2048,
            ids.shadow_map_size_button_left,
            ids.shadow_map_size_button_right,
            ids.shadow_map_size_text_canvas,
            ids.shadow_map_size_text,
            ids.shadow_map_size_canvas,
            ui,
        ) {
            self.set_force_redraw(true);
        };

        let mut bindings_option_index = 0;
        let any_waiting_for_keypress = self.wait_for_keypress_state.is_waiting();
        let mut redraw = false;
        let mut show_warning = false;
        let mut hide_warning = false;

        // Move forward binding option
        option_canvas(
            &mut bindings_option_index,
            ids.move_forward_canvas,
            ids.options_bindings_canvas,
            ui,
        );

        option_label(
            "Move forward",
            ids.move_forward_label,
            ids.move_forward_canvas,
            ui,
        );

        let bind_result = binding_option(
            &mut self.wait_for_keypress_state.move_forward,
            any_waiting_for_keypress,
            keypress,
            config::BindName::MoveForward,
            &mut self.new_config.bindings,
            ids.move_forward_canvas_2,
            ids.move_forward_button,
            ids.move_forward_rect,
            ids.move_forward_text,
            ids.move_forward_text_2,
            ids.move_forward_canvas,
            ui,
        );

        redraw = redraw || bind_result.redraw;
        show_warning = show_warning || bind_result.show_warning;
        hide_warning = hide_warning || bind_result.hide_warning;

        // Move left binding option
        option_canvas(
            &mut bindings_option_index,
            ids.move_left_canvas,
            ids.options_bindings_canvas,
            ui,
        );

        option_label(
            "Move left",
            ids.move_left_label,
            ids.move_left_canvas,
            ui,
        );

        let bind_result = binding_option(
            &mut self.wait_for_keypress_state.move_left,
            any_waiting_for_keypress,
            keypress,
            config::BindName::MoveLeft,
            &mut self.new_config.bindings,
            ids.move_left_canvas_2,
            ids.move_left_button,
            ids.move_left_rect,
            ids.move_left_text,
            ids.move_left_text_2,
            ids.move_left_canvas,
            ui,
        );

        redraw = redraw || bind_result.redraw;
        show_warning = show_warning || bind_result.show_warning;
        hide_warning = hide_warning || bind_result.hide_warning;

        // Move right binding option
        option_canvas(
            &mut bindings_option_index,
            ids.move_right_canvas,
            ids.options_bindings_canvas,
            ui,
        );

        option_label(
            "Move right",
            ids.move_right_label,
            ids.move_right_canvas,
            ui,
        );

        let bind_result = binding_option(
            &mut self.wait_for_keypress_state.move_right,
            any_waiting_for_keypress,
            keypress,
            config::BindName::MoveRight,
            &mut self.new_config.bindings,
            ids.move_right_canvas_2,
            ids.move_right_button,
            ids.move_right_rect,
            ids.move_right_text,
            ids.move_right_text_2,
            ids.move_right_canvas,
            ui,
        );

        redraw = redraw || bind_result.redraw;
        show_warning = show_warning || bind_result.show_warning;
        hide_warning = hide_warning || bind_result.hide_warning;

        // Move backward binding option
        option_canvas(
            &mut bindings_option_index,
            ids.move_backward_canvas,
            ids.options_bindings_canvas,
            ui,
        );

        option_label(
            "Move backward",
            ids.move_backward_label,
            ids.move_backward_canvas,
            ui,
        );

        let bind_result = binding_option(
            &mut self.wait_for_keypress_state.move_backward,
            any_waiting_for_keypress,
            keypress,
            config::BindName::MoveBackward,
            &mut self.new_config.bindings,
            ids.move_backward_canvas_2,
            ids.move_backward_button,
            ids.move_backward_rect,
            ids.move_backward_text,
            ids.move_backward_text_2,
            ids.move_backward_canvas,
            ui,
        );

        redraw = redraw || bind_result.redraw;
        show_warning = show_warning || bind_result.show_warning;
        hide_warning = hide_warning || bind_result.hide_warning;

        // This makes the warning persistent
        if show_warning {
            self.show_key_warning = true;
        }

        // Only disable the warning when a keybinding is successfully set
        if hide_warning {
            self.show_key_warning = false;
        }

        if redraw {
            self.set_force_redraw(true);
        }

        if self.show_key_warning {
            warning_text(
                "Key already in use",
                color::RED,
                ids.bind_in_use_warning_text,
                ids.options_bindings_canvas,
                ui,
            );
        }

        // Auto-revert window settings pop-up
        let redraw = if self.showing_auto_revert() {
            auto_revert_popup(
                &mut self.auto_revert_state,
                &mut self.current_config,
                &mut self.new_config,
                config,
                ids,
                ids.options_root_canvas,
                ui,
                event_channel,
            )
        } else {
            false
        };

        if redraw {
            self.set_force_redraw(true);
        }

        if update_config {
            // If the window dimensions or fullscreen options were changed, show the auto-revert
            // window settings pop-up
            if self.new_config.window.fullscreen != self.current_config.window.fullscreen ||
                self.new_config.window.dimensions != self.current_config.window.dimensions
            {
                self.auto_revert_state =
                    Some(AutoRevertState::new(
                            self.current_config.window.dimensions.clone(),
                            self.current_config.window.fullscreen,
                    ));
            }

            send_config_changed_events(&self.current_config, &self.new_config, event_channel);

            // Write the new config to the `Config` resource
            *config = self.new_config.clone().into();

            // Update `current_config` field
            let new_config = self.new_config.clone();

            self.current_config.camera = new_config.camera;
            self.current_config.graphics = new_config.graphics;
            self.current_config.bindings = new_config.bindings;
            self.current_config.window = new_config.window;

            // Force redraw to make "changes require restart" warning go away
            self.set_force_redraw(true);
        }

        if exit_options_menu {
            match self.options_menu_return_to.take() {
                Some(menu) => {
                    self.wait_for_keypress_state = WaitForKeypressState::new();
                    self.set_ui_state(ui_state, menu.into());
                }
                None => {
                    error!(log, "No menu to return to from options menu";);
                }
            }
        }
    }
}

/// Creates a button meant to transition between UI menus from the options menu
fn options_transition_button<'a>(
    button: widget::Button<'a, widget::button::Flat>,
    label: &'a str
) -> widget::Button<'a, widget::button::Flat> {
     button
        .w_h(OPTIONS_TRANSITION_BUTTON_WIDTH, 50.0)
        .color(consts::GENERIC_BUTTON_COLOR)
        .label(label)
        .label_font_size(consts::GENERIC_BUTTON_LABEL_FONT_SIZE)
        .label_color(consts::GENERIC_BUTTON_LABEL_COLOR)
}

/// Creates a canvas for an individual option
fn option_canvas(
    // The index of the option canvas from the top of the parent canvas, used to calculate position
    // of the canvas
    option_index: &mut u32,
    id: petgraph::graph::NodeIndex,
    parent: petgraph::graph::NodeIndex,
    ui: &mut conrod::UiCell,
) {
    widget::Canvas::new()
        .mid_top_with_margin_on(parent, OPTION_HEIGHT * f64::from(*option_index))
        .h(OPTION_HEIGHT)
        .w_of(parent)
        .color(OPTION_CANVAS_COLOR)
        .align_middle_x_of(parent)
        .set(id, ui);

    *option_index += 1;
}

/// Creates an option name text widget at the left of the parent widget
fn option_label(
    text: &str,
    id: petgraph::graph::NodeIndex,
    parent: petgraph::graph::NodeIndex,
    ui: &mut conrod::UiCell,
) {
    widget::Text::new(text)
        .mid_left_with_margin_on(parent, OPTION_MARGIN)
        .font_size(OPTION_NAME_FONT_SIZE)
        .color(OPTION_NAME_COLOR)
        .set(id, ui);
}

/// Creates and handles widgets for a `SelectOption` object
///
/// Returns `true` if the selection has changed and the UI should be redrawn
#[must_use]
fn option_selector<O: SelectOption + fmt::Display>(
    option: &mut O,
    max_option: O,
    id_button_left: petgraph::graph::NodeIndex,
    id_button_right: petgraph::graph::NodeIndex,
    id_text_canvas: petgraph::graph::NodeIndex,
    id_text: petgraph::graph::NodeIndex,
    parent: petgraph::graph::NodeIndex,
    ui: &mut conrod::UiCell,
) -> bool {
    let mut redraw = false;

    if widget::Button::new()
        .color(color::RED)
        .mid_right_with_margin_on(parent, OPTION_MARGIN)
        .w_h(50.0, OPTION_HEIGHT * 0.75)
        .label(">")
        .label_color(color::LIGHT_GRAY)
        .label_font_size(28)
        .set(id_button_right, ui)
        .was_clicked()
    {
        *option = option.next(max_option);
        redraw = true;
    }

    widget::Canvas::new()
        .color(color::RED)
        .align_middle_y_of(parent)
        .w(150.0)
        .x_relative(-150.0)
        .set(id_text_canvas, ui);

    widget::Text::new(&format!("{}", option))
        .align_middle_y_of(id_text_canvas)
        .align_middle_x_of(id_text_canvas)
        .font_size(OPTION_LABEL_FONT_SIZE)
        .set(id_text, ui);

    if widget::Button::new()
        .color(color::RED)
        .align_middle_y_of(parent)
        .x_relative(-150.0)
        .wh_of(id_button_right)
        .label("<")
        .label_color(color::LIGHT_GRAY)
        .label_font_size(28)
        .set(id_button_left, ui)
        .was_clicked()
    {
        *option = option.previous();
        redraw = true;
    }

    redraw
}

/// Creates and handles a toggle button widget
///
/// Returns `true` if the selection has changed and the UI should be redrawn
fn toggle_button(
    state: &mut bool,
    id: petgraph::graph::NodeIndex,
    parent: petgraph::graph::NodeIndex,
    extra_margin: conrod::Scalar,
    ui: &mut conrod::UiCell,
) -> bool {
    let text = if *state {
        "On"
    } else {
        "Off"
    };

    if widget::Button::new()
        .color(color::RED)
        .mid_right_with_margin_on(parent, OPTION_MARGIN + extra_margin)
        .w_h(150.0, OPTION_HEIGHT * 0.8)
        .label(text)
        .label_font_size(OPTION_LABEL_FONT_SIZE)
        .set(id, ui)
        .was_clicked()
    {
        *state = !*state;
        true
    } else {
        false
    }
}

/// Creates a text widget at the bottom of the parent widget to notify the user that changes made to
/// the configuration requires a restart
fn warning_text(
    text: &str,
    color: color::Color,
    id: petgraph::graph::NodeIndex,
    parent: petgraph::graph::NodeIndex,
    ui: &mut conrod::UiCell,
) {
    widget::Text::new(text)
        .color(color)
        .font_size(24)
        .mid_bottom_with_margin_on(parent, OPTION_MARGIN)
        .set(id, ui);
}

/// Returns a list of `ConfigChanged` events to send based on the differences between the two
/// `ConfigUiState`s
fn get_config_changed_events(a: &ConfigUiState, b: &ConfigUiState) -> Vec<window_event::Event> {
    let mut events = Vec::new();

    if a.graphics != b.graphics {
        events.push(window_event::Event::ConfigChanged(window_event::ChangedConfig::Graphics));
    }

    if a.window != b.window {
        events.push(window_event::Event::ConfigChanged(window_event::ChangedConfig::Window));
    }

    if a.camera != b.camera {
        events.push(window_event::Event::ConfigChanged(window_event::ChangedConfig::Camera));
    }

    if a.bindings != b.bindings {
        events.push(window_event::Event::ConfigChanged(window_event::ChangedConfig::Bindings));
    }

    events
}

/// Sends `ConfigChanged` events based on the differences between the two `ConfigUiState`s
pub fn send_config_changed_events(
    a: &ConfigUiState,
    b: &ConfigUiState,
    event_channel: &mut window_event::EventChannel,
) {
    for e in get_config_changed_events(a, b) {
        event_channel.single_write(e);
    }
}

/// Creates and handles the "auto revert window settings" pop-up
///
/// Returns `true` if the UI should be redrawn
#[must_use]
pub fn auto_revert_popup(
    auto_revert_state: &mut Option<AutoRevertState>,
    current_config: &mut ConfigUiState,
    new_config: &mut ConfigUiState,
    config_resource: &mut config::Config,
    ids: &Ids,
    parent: petgraph::graph::NodeIndex,
    ui: &mut conrod::UiCell,
    event_channel: &mut window_event::EventChannel,
) -> bool {
    let mut end_popup = false;

    if let Some(state) = auto_revert_state {
        let elapsed = state.popup_start_time.elapsed().as_secs();
        let mut revert = false;

        if elapsed >= AUTO_REVERT_TIME {
            revert = true;
            end_popup = true;
        }

        // A large, semi-transparent canvas to prevent interaction with other parts of the UI
        widget::Canvas::new()
            .color(color::Color::Rgba(0.0, 0.0, 0.0, 0.5))
            .wh_of(parent)
            .middle_of(parent)
            .set(ids.auto_revert_big_canvas, ui);

        // The canvas for the popup
        let (popup_width, popup_height) = (400.0, 300.0);

        widget::Canvas::new()
            .w_h(popup_width, popup_height)
            .middle_of(ids.auto_revert_big_canvas)
            .set(ids.auto_revert_canvas, ui);

        // A canvas to hold the title of the popup, to have a nice border
        widget::Canvas::new()
            .w_of(ids.auto_revert_canvas)
            .h(60.0)
            .mid_top_of(ids.auto_revert_canvas)
            .set(ids.keep_changes_canvas, ui);

        let auto_revert_text = &format!(
            "Auto-reverting changes in {} seconds",
            AUTO_REVERT_TIME - elapsed
        );

        widget::Text::new(auto_revert_text)
            .y_relative(-50.0)
            .w(popup_width * 0.9)
            .wrap_by_word()
            .center_justify()
            .set(ids.auto_revert_text, ui);

        let keep_changes_text = "Keep window size changes?";

        widget::Text::new(keep_changes_text)
            .middle_of(ids.keep_changes_canvas)
            .set(ids.keep_changes_text, ui);

        if widget::Button::new()
            .color(consts::GENERIC_BUTTON_COLOR)
            .w_h(popup_width * 0.4, popup_height * 0.25)
            .mid_left_with_margin_on(ids.auto_revert_canvas, 25.0)
            .down(100.0)
            .label("Keep changes")
            .label_font_size(22)
            .set(ids.keep_changes_button, ui)
            .was_clicked()
        {
            end_popup = true;
        }

        if widget::Button::new()
            .color(consts::GENERIC_BUTTON_COLOR)
            .wh_of(ids.keep_changes_button)
            .mid_right_with_margin_on(ids.auto_revert_canvas, 25.0)
            .y_relative(0.0)
            .label("Revert changes")
            .label_font_size(22)
            .set(ids.revert_changes_button, ui)
            .was_clicked()
        {
            revert = true;
            end_popup = true;
        }

        if revert {
            // Edit the new config values
            new_config.window.dimensions = state.old_dimensions.clone();
            new_config.window.fullscreen = state.old_fullscreen;

            // Write to the `Config` resource
            config_resource.window = new_config.window.clone().into();

            // Send `ConfigChanged` events
            send_config_changed_events(current_config, new_config, event_channel);

            // Update the current config values as well
            current_config.window.dimensions = state.old_dimensions.clone();
            current_config.window.fullscreen = state.old_fullscreen;
        }
    }

    if end_popup {
        *auto_revert_state = None;
    }

    end_popup
}

/// Represents what should happen as a result of handling a keybinding editor widget
struct BindResult {
    redraw: bool,
    show_warning: bool,
    hide_warning: bool,
}

impl BindResult {
    fn new() -> Self {
        Self {
            redraw: false,
            show_warning: false,
            hide_warning: false,
        }
    }
}

/// Creates and handles a keybinding editor widget
#[must_use]
fn binding_option(
    // Whether this keybinding widget is waiting for a keypress
    waiting_for_keypress: &mut bool,
    // Whether any keybinding widget is waiting for a keypress
    any_waiting_for_keypress: bool,
    keypress: Option<glutin::KeyboardInput>,
    binding: config::BindName,
    bindings: &mut config::BindConfig,
    canvas_id: petgraph::graph::NodeIndex,
    button_id: petgraph::graph::NodeIndex,
    rect_id: petgraph::graph::NodeIndex,
    text_id: petgraph::graph::NodeIndex,
    text_id_2: petgraph::graph::NodeIndex,
    parent: petgraph::graph::NodeIndex,
    ui: &mut conrod::UiCell,
) -> BindResult {
    let mut result = BindResult::new();
    let (w, h) = (200.0, OPTION_HEIGHT * 0.8);

    if *waiting_for_keypress  {
        widget::Canvas::new()
            .w_h(w, h)
            .color(color::RED.clicked())
            .mid_right_with_margin_on(parent, OPTION_MARGIN)
            .set(rect_id, ui);

        widget::Canvas::new()
            .w_h(w, h)
            .y_relative(0.0)
            .x_relative(-w)
            .set(canvas_id, ui);

        widget::Text::new("Press any key...")
            .middle_of(rect_id)
            .set(text_id, ui);

        if let Some(input) = keypress {
            if let Some(keycode) = input.virtual_keycode {
                match keycode {
                    glutin::VirtualKeyCode::Escape => {
                        // If `Escape` was pressed, stop editing this keybinding and hide the "key
                        // in use" warning if it is showing
                        result.hide_warning = true;
                        result.redraw = true;
                        *waiting_for_keypress = false;
                    }
                    glutin::VirtualKeyCode::LAlt |
                    glutin::VirtualKeyCode::RAlt |
                    glutin::VirtualKeyCode::LControl |
                    glutin::VirtualKeyCode::RControl |
                    glutin::VirtualKeyCode::LShift |
                    glutin::VirtualKeyCode::RShift |
                    glutin::VirtualKeyCode::LWin |
                    glutin::VirtualKeyCode::RWin
                    => {
                        // The above keys cannot be bound
                    }
                    _ => {
                        let new_bind = config::Bind::new(keycode.into(), input.modifiers.into());

                        // If the keybinding is in use, show a warning and continue waiting for a
                        // keypress
                        if bindings.is_in_use(&new_bind) {
                            result.show_warning = true;
                        } else {
                            // Otherwise, hide the warning if it is showing and set the keybinding
                            result.hide_warning = true;
                            bindings.set(binding.clone(), new_bind);
                            *waiting_for_keypress = false;
                        };

                        result.redraw = true;
                    }
                }
            }
        }
    } else {
        if widget::Button::new()
            .color(color::RED)
            .mid_right_with_margin_on(parent, OPTION_MARGIN)
            .w_h(w, h)
            .label("Change")
            .set(button_id, ui)
            .was_clicked()
        {
            if !any_waiting_for_keypress {
                *waiting_for_keypress = true;
                result.redraw = true;
            }
        }

        widget::Canvas::new()
            .w_h(w, h)
            .y_relative(0.0)
            .x_relative(-w)
            .set(canvas_id, ui);
    }

    widget::Text::new(&format!("{}", bindings.get_mut(binding)))
        .middle_of(canvas_id)
        .set(text_id_2, ui);

    result
}
