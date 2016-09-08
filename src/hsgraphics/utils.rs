use consts::graphics::*;

pub fn normalize_xy(d: f32) -> (f32, f32) {
    (d / WINDOW_WIDTH as f32,
     d / WINDOW_HEIGHT as f32)
}
