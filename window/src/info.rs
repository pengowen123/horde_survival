//! Properties of the window for use in graphics-related systems, and a system to update a resource
//! containing those properties

use common::{self, glutin, specs};

#[derive(Clone, Copy, Debug)]
pub struct WindowInfo {
    dimensions: glutin::dpi::LogicalSize,
    aspect_ratio: f32,
    dpi: f32,
}

impl Default for WindowInfo {
    fn default() -> Self {
        Self {
            dimensions: glutin::dpi::LogicalSize::new(100.0, 100.0),
            aspect_ratio: 1.0,
            dpi: 1.0,
        }
    }
}

impl WindowInfo {
    pub fn new(window: &glutin::Window) -> Self {
        let dpi = window.get_hidpi_factor();
        let dimensions = match window.get_inner_size() {
            Some(d) => d,
            // This should only happen when the window gets closed, so it's okay to return a
            // default value
            None => return Default::default(),
        };

        let aspect_ratio = dimensions.width as f32 / dimensions.height as f32;

        Self {
            dimensions,
            aspect_ratio,
            dpi: dpi as f32,
        }
    }

    pub fn dimensions(&self) -> glutin::dpi::LogicalSize {
        self.dimensions
    }

    pub fn physical_dimensions(&self) -> glutin::dpi::PhysicalSize {
        self.dimensions.to_physical(self.dpi.into())
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn dpi(&self) -> f32 {
        self.dpi
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
