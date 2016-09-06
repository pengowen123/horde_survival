pub struct GraphicsState {
    pub window_position: (i32, i32),
    pub window_center: (i32, i32),
}

impl GraphicsState {
    pub fn new() -> GraphicsState {
        GraphicsState {
            window_position: (0, 0),
            window_center: (0, 0),
        }
    }
}
