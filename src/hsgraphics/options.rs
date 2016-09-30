#[derive(Clone)]
pub struct GraphicsOptions {
    pub minimap_enabled: bool,
    pub display_debug: bool,
    pub crosshair: bool,
    pub fullscreen: bool,
}

impl GraphicsOptions {
    pub fn new() -> GraphicsOptions {
        GraphicsOptions {
            minimap_enabled: false,
            display_debug: false,
            crosshair: false,
            fullscreen: false,
        }
    }
}

impl GraphicsOptions {
    pub fn minimap_enabled(&mut self, value: bool) -> &mut GraphicsOptions {
        self.minimap_enabled = value;
        self
    }

    pub fn display_debug(&mut self, value: bool) -> &mut GraphicsOptions {
        self.display_debug = value;
        self
    }

    pub fn crosshair(&mut self, value: bool) -> &mut GraphicsOptions {
        self.crosshair = value;
        self
    }

    // NOTE: Don't set this to true, it crashes on window creation when using fullscreen
    pub fn fullscreen(&mut self, value: bool) -> &mut GraphicsOptions {
        self.fullscreen = value;
        self
    }
}
