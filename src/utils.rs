use glutin::{Window, CursorState};

use std::time::Duration;

pub fn millis(duration: Duration) -> u64 {
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos() as u64;

    secs * 1000 + nanos / 1_000_000
}

pub fn set_cursor_state(window: &Window, cursor_state: CursorState) {
    if let Err(_) = window.set_cursor_state(cursor_state) {
        warn!("Failed to set cursor state to {:?}", cursor_state);
    }
}
