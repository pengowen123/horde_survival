#[derive(Clone)]
pub struct GraphicsOptions {
    pub minimap_enabled: bool,
}

impl GraphicsOptions {
    pub fn new() -> GraphicsOptions {
        GraphicsOptions {
            minimap_enabled: false,
        }
    }
}

impl GraphicsOptions {
    pub fn minimap_enabled(&mut self, value: bool) -> &mut GraphicsOptions {
        self.minimap_enabled = value;
        self
    }
}
