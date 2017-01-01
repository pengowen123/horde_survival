use consts::defaults::*;

#[derive(Clone)]
pub struct GraphicsOptions {
    pub window_size: (u32, u32),
    pub minimap_enabled: bool,
    pub display_debug: bool,
    pub crosshair: bool,
    pub fullscreen: bool,
}

impl GraphicsOptions {
    pub fn new() -> GraphicsOptions {
        GraphicsOptions {
            window_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            minimap_enabled: false,
            display_debug: false,
            crosshair: false,
            fullscreen: false,
        }
    }
}

impl GraphicsOptions {
    pub fn window_size(&mut self, width: u32, height: u32) -> &mut Self {
        self.window_size = (width, height);
        self
    }

    pub fn minimap_enabled(&mut self, value: bool) -> &mut Self {
        self.minimap_enabled = value;
        self
    }

    pub fn display_debug(&mut self, value: bool) -> &mut Self {
        self.display_debug = value;
        self
    }

    pub fn crosshair(&mut self, value: bool) -> &mut Self {
        self.crosshair = value;
        self
    }

    pub fn fullscreen(&mut self, value: bool) -> &mut Self {
        self.fullscreen = value;
        self
    }
}
