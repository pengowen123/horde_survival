use glutin::Window;

use hsgraphics::GraphicsState;

/// Moves the cursor to the center of the window
pub fn center_mouse(state: &mut GraphicsState, mouse: &mut (i32, i32), window: &Window) {
    let (x, y) = state.window_center;

    // Measure the cursor's offset from the window center
    mouse.0 = state.last_cursor_pos.0 - x;
    mouse.1 = state.last_cursor_pos.1 - y;

    state.last_cursor_pos = state.window_center;

    // Move the cursor
    if window.set_cursor_position(x, y).is_err() {
        error!("Failed to set cursor position to ({}, {})", x, y);
    }
}
