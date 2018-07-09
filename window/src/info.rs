//! Properties of the window for use in graphics-related systems, and a system to update a resource
//! containing those properties

use common::{self, glutin, specs};

#[derive(Clone, Copy, Debug)]
pub struct WindowInfo {
    dimensions: (u32, u32),
    aspect_ratio: f32,
}

impl Default for WindowInfo {
    fn default() -> Self {
        Self {
            dimensions: (100, 100),
            aspect_ratio: 1.0,
        }
    }
}

impl WindowInfo {
    pub fn new(window: &glutin::Window) -> Self {
        let dimensions = match window.get_inner_size() {
            Some(d) => d,
            // This should only happen when the window gets closed, so it's okay to return a
            // default value
            None => return Default::default(),
        };

        let aspect_ratio = dimensions.0 as f32 / dimensions.1 as f32;

        Self {
            dimensions,
            aspect_ratio,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
}

pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    window_info: specs::FetchMut<'a, WindowInfo>,
    window: specs::Fetch<'a, ::Window>,
    // TODO: remove when a better way of displaying this info is implemented
    delta: specs::Fetch<'a, common::Delta>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        *data.window_info = WindowInfo::new(&data.window);

        data.window.set_title(&format!(
            "Horde Survival - {:.4} ms",
            data.delta.to_float() * 1000.0,
        ));
    }
}
