//! Systems and components related to graphics and the window

// TODO: Remove pub when a better way to create drawables is made (such as a obj loading system)
pub mod draw;
mod window;
mod camera;

use gfx;
use gfx::traits::FactoryExt;
use gfx_window_glutin;
use glutin::{self, EventsLoop, GlContext};
use specs::{self, DispatcherBuilder};

use std::sync::Arc;

const VERTEX_SHADER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/shaders/vertex_150.glsl"
));
const FRAGMENT_SHADER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/shaders/fragment_150.glsl"
));

/// Initializes graphics-related components and systems
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> (DispatcherBuilder<'a, 'b>, window::Window, EventsLoop) {
    // Initialize window settings
    let (w, h) = (800, 600);
    let events = EventsLoop::new();
    let context_builder = glutin::ContextBuilder::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Horde Survival")
        .with_min_dimensions(w, h);

    // Initialize gfx structs
    let (window, device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<draw::ColorFormat, draw::DepthFormat>(
            window_builder,
            context_builder,
            &events,
        );

    unsafe {
        window.make_current().unwrap();
    }

    let window = Arc::new(window);
    let encoder = factory.create_command_buffer().into();
    let pso = factory
        .create_pipeline_simple(VERTEX_SHADER, FRAGMENT_SHADER, draw::init_pipeline())
        .unwrap_or_else(|e| panic!("Failed to create PSO: {}", e));

    // Register components
    register_drawable(world, &factory);

    // Add test entities
    // TODO: Remove this when the game has a better initialization system
    ::dev::add_test_entities(world, &mut factory);

    // Add resources
    world.add_resource(window::WindowInfo::new(&window));
    world.add_resource(camera::Camera::new_default(w as f32 / h as f32));
    world.add_resource(window.clone());

    let system = draw::System::new(factory, device, main_color, main_depth, encoder, pso);
    let dispatcher = dispatcher
        .add(window::System, "window-info", &[])
        .add(camera::System, "camera", &["window-info"])
        .add_thread_local(system);

    (dispatcher, window, events)
}

/// A hack to register `Drawable<R>` without specifying the type of `R`
fn register_drawable<R, F>(world: &mut specs::World, _: &F)
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    world.register::<draw::Drawable<R>>();
}
