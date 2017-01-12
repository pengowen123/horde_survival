use glutin::{Event, Window, CursorState};

use std::time::Duration;

pub fn millis(duration: Duration) -> u64 {
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos() as u64;

    secs * 1000 + nanos / 1_000_000
}

pub fn set_cursor_state(window: &Window, cursor_state: CursorState) {
    window.set_cursor_state(cursor_state)
        .unwrap_or_else(|e| warn!("Failed to set cursor state to {:?} ({})", cursor_state, e))
}

pub fn is_mouse_moved_event(event: &Event) -> bool {
    if let Event::MouseMoved(..) = *event {
        true
    } else {
        false
    }
}

pub fn clamp<T: PartialOrd>(t: T, lower: T, upper: T) -> T {
    if t < lower {
        lower
    } else if t > upper {
        upper
    } else {
        t
    }
}

macro_rules! log_create_pso {
    ($name:expr, $vs_path:expr, $fs_path:expr) => {{
        info!("Creating PSO: {} (vertex: {}, fragment: {})", $name, $vs_path, $fs_path);
    }};
}
