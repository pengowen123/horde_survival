//! Constants for the UI
//!
//! Constants are used to keep UI elements with similar purposes consistent in appearance

use common::conrod::widget::{self, button};
use common::conrod::{self, color, Colorable, Labelable, Sizeable};

pub const UI_BACKGROUND_COLOR: conrod::Color = color::LIGHT_CHARCOAL;
pub const GENERIC_BUTTON_WIDTH: conrod::Scalar = 300.0;
pub const GENERIC_BUTTON_HEIGHT: conrod::Scalar = 50.0;
pub const GENERIC_BUTTON_COLOR: conrod::Color = color::DARK_GRAY;
pub const GENERIC_BUTTON_LABEL_COLOR: conrod::Color = color::LIGHT_GRAY;
pub const GENERIC_BUTTON_LABEL_FONT_SIZE: u32 = 28;
/// The distance between the start of each button flowing downward
pub const GENERIC_BUTTON_SPACING: conrod::Scalar = GENERIC_BUTTON_HEIGHT * -1.2;

/// Creates a button with the properties specified by the `GENERIC_BUTTON_*` constants
pub fn create_generic_button<'a>(
    button: widget::Button<'a, button::Flat>,
    label: &'a str,
) -> widget::Button<'a, button::Flat> {
    button
        .w_h(GENERIC_BUTTON_WIDTH, GENERIC_BUTTON_HEIGHT)
        .color(GENERIC_BUTTON_COLOR)
        .label(label)
        .label_font_size(GENERIC_BUTTON_LABEL_FONT_SIZE)
        .label_color(GENERIC_BUTTON_LABEL_COLOR)
}
