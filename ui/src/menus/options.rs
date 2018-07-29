//! Implementation of the options menu

use common::{UiState, gfx, config};
use common::conrod::{self, Colorable, Positionable, Sizeable, Labelable, color};
use common::conrod::widget::{self, Widget};
use window::window_event;
use slog;
use petgraph;

use std::cmp;

use menus::Menus;
use consts::{self, UI_BACKGROUND_COLOR};

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

#[derive(Clone, PartialEq)]
pub enum ShadowMapSize {
    _512,
    _1024,
    _2048,
    Custom(gfx::texture::Size),
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
            WindowDimensions::_1440x1080 => 1,
            WindowDimensions::_1920x1080 => 2,
            WindowDimensions::Custom(..) => -1,
        }
    }

    /// Returns the next window size from this one
    ///
    /// Returns `self` if it is already the maximum window size
    fn next_size(&self, max_window_size: WindowDimensions) -> WindowDimensions {
        if let WindowDimensions::Custom(..) = *self {
            // There isn't anything to do here that makes sense other than return a default value
            WindowDimensions::_800x600
        } else if self >= &max_window_size {
            self.clone()
        } else {
            match *self {
                WindowDimensions::_800x600 => WindowDimensions::_1440x1080,
                WindowDimensions::_1440x1080 => WindowDimensions::_1920x1080,
                WindowDimensions::_1920x1080 => WindowDimensions::_1920x1080,
                WindowDimensions::Custom(..) => unreachable!(),
            }
        }
    }

    /// Returns the previous window size from this one
    ///
    /// Returns `self` if it is already the minimum window size
    fn previous_size(&self) -> WindowDimensions {
        match *self {
            WindowDimensions::_800x600 => WindowDimensions::_800x600,
            WindowDimensions::_1440x1080 => WindowDimensions::_800x600,
            WindowDimensions::_1920x1080 => WindowDimensions::_1440x1080,
            // There isn't anything to do here that makes sense other than return a default value
            WindowDimensions::Custom(..) => WindowDimensions::_800x600,
        }
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
            .color(color::PURPLE)
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
            (ids.options_window_canvas, "Graphics"),
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

        widget::Text::new("Field of view")
            .mid_left_with_margin_on(ids.fov_canvas, OPTION_MARGIN)
            .font_size(OPTION_NAME_FONT_SIZE)
            .color(OPTION_NAME_COLOR)
            .set(ids.fov_label, ui);

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

        widget::Text::new("Sensitivity")
            .mid_left_with_margin_on(ids.sensitivity_canvas, OPTION_MARGIN)
            .font_size(OPTION_NAME_FONT_SIZE)
            .color(OPTION_NAME_COLOR)
            .set(ids.sensitivity_label, ui);

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

        if update_config {
            // Send `ConfigChanged` events
            let events = get_config_changed_events(&self.current_config, &self.new_config);

            for e in events {
                event_channel.single_write(e);
            }

            // Write the new config to the `Config` resource
            *config = self.new_config.clone().into();

            // Update `current_config` field
            self.current_config = self.new_config.clone();
        }

        if exit_options_menu {
            match self.options_menu_return_to.take() {
                Some(menu) => {
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
