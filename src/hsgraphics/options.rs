use consts::defaults::*;

/// Graphics options
#[derive(Clone)]
pub struct GraphicsOptions {
    pub window_size: (u32, u32),
    pub display_debug: bool,
    pub crosshair: bool,
    pub fullscreen: bool,
}

impl Default for GraphicsOptions {
    fn default() -> Self {
        GraphicsOptions {
            window_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            display_debug: false,
            crosshair: false,
            fullscreen: false,
        }
    }
}

impl GraphicsOptions {
    pub fn new() -> GraphicsOptions {
        Default::default()
    }

    /// Sets the window size
    pub fn window_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_size = (width, height);
        self
    }

    /// Sets whether to display debug info

    // NOTE: This may be repurposed in the future when the current debug info display is changed
    pub fn display_debug(&mut self, value: bool) -> &mut Self {
        self.display_debug = value;
        self
    }

    /// Sets whether to display a crosshair in the middle of the screen while playing
    pub fn crosshair(&mut self, value: bool) -> &mut Self {
        self.crosshair = value;
        self
    }

    /// Sets whether to make the window fullscreen
    pub fn fullscreen(&mut self, value: bool) -> &mut Self {
        self.fullscreen = value;
        self
    }
}
