//! Common utilities

use glutin::{self, dpi};

/// Returns the center point of the window
pub fn get_window_center(window: &glutin::Window) -> Option<dpi::LogicalPosition> {
    let window_size = window.get_inner_size()?;

    let center = dpi::LogicalPosition::new(window_size.width / 2.0, window_size.height / 2.0);

    Some(center)
}

/// Sets the cursor to the center of the window
pub fn set_cursor_pos_to_window_center(window: &glutin::Window) {
    let center = get_window_center(window).expect("Failed to get window center");
    window
        .set_cursor_position(center)
        .expect("Failed to set cursor position");
}
