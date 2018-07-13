//! Initialization of the rendering system

use specs::{self, DispatcherBuilder};
use common::glutin::{self, EventsLoop, GlContext};
use common;
use gfx_window_glutin;
use gfx;
use window;
use slog;
use ui;

use std::sync::{Arc, Mutex};

use super::{param, components, lighting_data, passes};

use gfx_device_gl;
/// Initializes rendering-related components and systems
pub fn initialize<'a, 'b, 'c, 'd>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
    dispatcher_graphics: DispatcherBuilder<'c, 'd>,
    init_test_entities: Box<Fn(&mut specs::World, &mut gfx_device_gl::Factory)>,
) -> (DispatcherBuilder<'a, 'b>,
      DispatcherBuilder<'c, 'd>,
      window::Window,
      EventsLoop)
{

    // Initialize window settings
    let events = EventsLoop::new();
    let context_builder = glutin::ContextBuilder::new();
    let window_builder = {
        let size = glutin::dpi::LogicalSize::new(800.0, 600.0);
        glutin::WindowBuilder::new()
            .with_title("Horde Survival")
            .with_min_dimensions(size)
    };

    // Initialize gfx structs
    let (window, device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<super::ColorFormat, super::DepthFormat>(
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
    world.add_resource(Arc::new(Mutex::new(passes::shadow::DirShadowSource::new_none())));

    // Initialize subsystems
    let dispatcher = param::init(world, dispatcher);
    let dispatcher = lighting_data::init(world, dispatcher);

    // Add test entities
    // TODO: Remove this when the game has a better initialization system
    // NOTE: This line must come after registering all required components; move it around to
    //       satisfy this as needed
    init_test_entities(world, &mut factory);

    // Initialize systems
    let draw = super::System::new(
            factory,
            window.clone(),
            device,
            main_color,
            main_depth,
            encoder,
            &mut world.res,
    );

    // Add systems
    let dispatcher = dispatcher
        .add(
            param::System::new(draw.factory()),
            "shader-param",
            &[
                "shader-param-translation",
                "shader-param-rotation",
                "shader-param-scale",
            ],
        )
        .add(
            passes::shadow::ShadowSourceSystem,
            "shadow-source",
            &[],
            );

    let dispatcher_graphics = dispatcher_graphics.add_thread_local(draw);

    (dispatcher, dispatcher_graphics, window, events)
}

/// A hack to register components with a `gfx::Resource` type parameter
fn register_components<R, F>(world: &mut specs::World, _: &F)
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
{
    world.register::<components::Drawable<R>>();
    world.add_resource(ui::ImageMap::<R>::new())
}
