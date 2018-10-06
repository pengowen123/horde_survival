//! Initialization of the rendering system

use common::glutin::{self, EventsLoop, GlContext};
use common::graphics::Drawable;
use common::{self, config};
use gfx;
use gfx_window_glutin;
use slog;
use specs::{self, DispatcherBuilder};
use ui;
use window;

use std::sync::{Arc, Mutex};

use draw::{self, components, lighting_data, param, passes};

use gfx_device_gl;
/// Initializes rendering-related components and systems
pub fn initialize<'a, 'b, 'c, 'd>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
    dispatcher_graphics: DispatcherBuilder<'c, 'd>,
    init_test_entities: Box<Fn(&mut specs::World, &mut gfx_device_gl::Factory)>,
) -> (
    DispatcherBuilder<'a, 'b>,
    DispatcherBuilder<'c, 'd>,
    window::Window,
    EventsLoop,
) {
    // Initialize window settings
    let events = EventsLoop::new();
    let (window_builder, context_builder) = {
        let config = world.read_resource::<config::Config>();
        let context_builder = glutin::ContextBuilder::new().with_vsync(config.window.vsync);
        let size =
            glutin::dpi::LogicalSize::new(config.window.width as f64, config.window.height as f64);
        let fullscreen = if config.window.fullscreen {
            Some(events.get_primary_monitor())
        } else {
            None
        };
        let window_builder = glutin::WindowBuilder::new()
            .with_title("Horde Survival")
            .with_min_dimensions(size)
            .with_dimensions(size)
            .with_fullscreen(fullscreen)
            .with_resizable(false);

        (window_builder, context_builder)
    };

    // Initialize gfx structs
    let (window, device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<draw::ColorFormat, draw::DepthFormat>(
            window_builder,
            context_builder,
            &events,
        );

    {
        let log = world.read_resource::<slog::Logger>();
        unsafe {
            window.make_current().unwrap_or_else(|e| {
                error!(log, "Failed to set make GL context the current one: {}", e;);
                panic!(common::CRASH_MSG);
            });
        }
    }

    let window = Arc::new(window);
    let encoder = factory.create_command_buffer().into();

    // Register components
    register_components(world, &factory);
    world.register::<components::DirectionalLight>();
    world.register::<components::PointLight>();
    world.register::<components::SpotLight>();

    // Add resources
    world.add_resource(Arc::new(Mutex::new(
        passes::shadow::DirShadowSource::new_none(),
    )));

    // Initialize subsystems
    let dispatcher = param::init(world, dispatcher);
    let dispatcher = lighting_data::init(world, dispatcher);

    // Add test entities
    // TODO: Remove this when the game has a better initialization system
    // NOTE: This line must come after registering all required components; move it around to
    //       satisfy this as needed
    init_test_entities(world, &mut factory);

    // Initialize systems
    let create_new_window_views = |window: &glutin::GlWindow| gfx_window_glutin::new_views(window);
    let create_new_window_views = Box::new(create_new_window_views);
    let draw = draw::System::new(
        factory,
        window.clone(),
        device,
        main_color,
        main_depth,
        encoder,
        &mut world.res,
        create_new_window_views,
    );

    // Add systems
    let dispatcher = dispatcher
        .with(
            param::System::new(draw.factory()),
            "shader-param",
            &[
                "shader-param-translation",
                "shader-param-rotation",
                "shader-param-scale",
            ],
        ).with(passes::shadow::ShadowSourceSystem, "shadow-source", &[]);

    let dispatcher_graphics = dispatcher_graphics.with_thread_local(draw);

    (dispatcher, dispatcher_graphics, window, events)
}

/// A hack to register components with a `gfx::Resource` type parameter
fn register_components<R, F>(world: &mut specs::World, _: &F)
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    world.register::<Drawable<R>>();
    world.add_resource(ui::ImageMap::<R>::new())
}
