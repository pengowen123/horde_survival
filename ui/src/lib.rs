//! A UI system
//!
//! The UI is only calculated here; a draw list is sent to the graphics system to be rendered

extern crate specs;
extern crate shred;
#[macro_use]
extern crate shred_derive;
extern crate common;

use common::conrod::render;
use common::conrod::{Ui, UiBuilder, Dimensions};

pub struct System {
    ui: Ui,
}

impl System {
    fn new(window_dim: Dimensions) -> Self {
        let ui = UiBuilder::new(window_dim).build();

        Self {
            ui
        }
    }
}

#[derive(SystemData)]
pub struct Data<'a> {
    draw_list: specs::FetchMut<'a, UiDrawList>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;
    fn run(&mut self, data: Self::SystemData) {
        //self.ui.t
    }
}

/// The UI represented as a list of objects to draw
pub struct UiDrawList(Option<render::OwnedPrimitives>);

impl UiDrawList {
    /// Returns a iterator-like object over the primitives in the draw list
    pub fn walk(&self) -> render::WalkOwnedPrimitives {
        self.0.as_ref().unwrap().walk()
    }
}

/// Adds UI-related resources
pub fn add_resources(world: &mut specs::World) {
    world.add_resource(UiDrawList(None));
}
/// Initializes UI-related systems
pub fn initialize(world: &mut specs::World, window_dim: (u32, u32)) -> System {
    let window_dim: Dimensions = [window_dim.0.into(), window_dim.1.into()];
    System::new(window_dim)
}
