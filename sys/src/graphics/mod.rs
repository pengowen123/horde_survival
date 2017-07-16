//! Systems and components related to graphics and the window

// TODO: Remove pub when a better way to create drawables is made (such as an obj loading system)
pub mod draw;
mod window;
mod camera;

use glutin;
use specs::{self, DispatcherBuilder};

/// Initializes graphics-related components and systems
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> (DispatcherBuilder<'a, 'b>, window::Window, glutin::EventsLoop) {
    // Initialize subsystems
    let (dispatcher, window, events) = draw::init(world, dispatcher);

    // Add resources
    let (w, h) = window.get_inner_size_pixels().unwrap();
    world.add_resource(camera::Camera::new_default(w as f32 / h as f32));
    world.add_resource(window::WindowInfo::new(&window));
    world.add_resource(window.clone());

    // Add systems
    let dispatcher = dispatcher
        // This comment is here to stop rustfmt from messing with these lines
        .add(window::System, "window-info", &[])
        .add(camera::System, "camera", &["window-info"]);

    (dispatcher, window, events)
}
