//! Themes for the UI

use common::conrod::{Theme, color};

use std::time::Duration;

pub fn default_theme() -> Theme {
    use conrod::position::{Align, Padding, Position, Relative};
    Theme {
        name: "Default Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Align(Align::Start), None),
        background_color: color::LIGHT_CHARCOAL,
        shape_color: color::LIGHT_RED,
        border_color: color::BLACK,
        border_width: 1.0,
        label_color: color::LIGHT_GRAY,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 20,
        font_size_small: 14,
        widget_styling: ::common::conrod::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: Duration::from_millis(500),
    }
}
