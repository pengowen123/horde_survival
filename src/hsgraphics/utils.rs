use consts::graphics::*;

pub fn get_scales(d: f32) -> (f32, f32) {
    (d * MINIMAP_SCALE / WINDOW_WIDTH as f32,
     d * MINIMAP_SCALE / WINDOW_HEIGHT as f32)
}
