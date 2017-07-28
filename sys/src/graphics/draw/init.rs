//! Initialization of the rendering system

use specs::{self, DispatcherBuilder};
use glutin::{self, EventsLoop, GlContext};
use gfx_window_glutin;
use gfx;

use std::sync::Arc;

use window;
use super::param;
use super::pipeline::{self, main, postprocessing};

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
    let (window, device, mut factory, main_color, _) =
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

    let main_vs_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/shaders/vertex_150.glsl"
    );
    let main_fs_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/shaders/fragment_150.glsl"
    );
    let pso_main = pipeline::load_pso(&mut factory, main_vs_path, main_fs_path, main::pipe::new())
        .unwrap_or_else(|e| panic!("Failed to create main PSO: {}", e));

    let post_vs_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/shaders/post_vertex_150.glsl"
    );
    let post_fs_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/shaders/post_fragment_150.glsl"
    );
    let pso_post = pipeline::load_pso(
        &mut factory,
        post_vs_path,
        post_fs_path,
        postprocessing::pipe::new(),
    ).unwrap_or_else(|e| panic!("Failed to create postprocessing PSO: {}", e));

    // Register components
    register_drawable(world, &factory);

    // Add test entities
    // TODO: Remove this when the game has a better initialization system
    // NOTE: This line must come after registering all required components; move it around to
    //       satisfy this as needed
    ::dev::add_test_entities(world, &mut factory);

    // Initialize systems
    let draw = super::System::new(
        factory,
        &*window,
        device,
        main_color,
        encoder,
        pso_main,
        pso_post,
    );

    // Add systems
    let dispatcher = dispatcher
        .add(
            param::System::new(draw.factory()),
            "shader-param",
            &["shader-param-translation", "shader-param-rotation"],
        )
        .add_thread_local(draw);

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
