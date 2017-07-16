//! Initialization of the rendering system

use specs::{self, DispatcherBuilder};
use glutin::{self, EventsLoop, GlContext};
use gfx_window_glutin;
use gfx;
use gfx::traits::FactoryExt;

use std::sync::Arc;

use graphics::window;
use super::param;

const VERTEX_SHADER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/shaders/vertex_150.glsl"
));
const FRAGMENT_SHADER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/shaders/fragment_150.glsl"
));

/// Initializes rendering-related components and systems
pub fn init<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> (DispatcherBuilder<'a, 'b>, window::Window, EventsLoop) {

    // Initialize subsystems
    let dispatcher = param::init(world, dispatcher);

    // Initialize window settings
    let (w, h) = (800, 600);
    let events = EventsLoop::new();
    let context_builder = glutin::ContextBuilder::new();
    let window_builder = glutin::WindowBuilder::new()
        .with_title("Horde Survival")
        .with_min_dimensions(w, h);

    // Initialize gfx structs
    let (window, device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<super::ColorFormat, super::DepthFormat>(
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
        .create_pipeline_simple(VERTEX_SHADER, FRAGMENT_SHADER, super::init_pipeline())
        .unwrap_or_else(|e| panic!("Failed to create PSO: {}", e));

    // Register components
    register_drawable(world, &factory);

    // Add test entities
    // TODO: Remove this when the game has a better initialization system
    // NOTE: This line must come after registering all required components; move it around to
    //       satisfy this as needed
    ::dev::add_test_entities(world, &mut factory);

    // Initialize systems
    let system = super::System::new(factory, device, main_color, main_depth, encoder, pso);

    // Add systems
    let dispatcher = dispatcher.add_thread_local(system);

    (dispatcher, window, events)
}

/// A hack to register `Drawable<R>` without specifying the type of `R`
fn register_drawable<R, F>(world: &mut specs::World, _: &F)
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    world.register::<super::Drawable<R>>();
}
