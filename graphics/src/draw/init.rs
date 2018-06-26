//! Initialization of the rendering system

use specs::{self, DispatcherBuilder};
use common::glutin::{self, EventsLoop, GlContext};
use gfx_window_glutin;
use gfx;
use window;

use std::sync::{Arc, Mutex};

use super::{param, components, lighting_data, passes};
use camera;

use gfx_device_gl;
/// Initializes rendering-related components and systems
pub fn initialize<'a, 'b>(
    world: &mut specs::World,
    dispatcher: DispatcherBuilder<'a, 'b>,
    init_test_entities: Box<Fn(&mut specs::World, &mut gfx_device_gl::Factory)>,
) -> (DispatcherBuilder<'a, 'b>, window::Window, EventsLoop) {

    // Initialize window settings
    let events = EventsLoop::new();
    let context_builder = glutin::ContextBuilder::new();
    let window_builder = {
        let (w, h) = (800, 600);
        glutin::WindowBuilder::new()
            .with_title("Horde Survival")
            .with_min_dimensions(w, h)
    };

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

    // Register components
    register_drawable(world, &factory);
    world.register::<components::DirectionalLight>();
    world.register::<components::PointLight>();
    world.register::<components::SpotLight>();

    // Add resources
    world.add_resource(Arc::new(Mutex::new(passes::shadow::DirShadowSource::new_none())));

    // Initialize subsystems
    let dispatcher = param::init(world, dispatcher);
    let (dispatcher, point_send, spot_send) = lighting_data::init(world, dispatcher);

    // Add test entities
    // TODO: Remove this when the game has a better initialization system
    // NOTE: This line must come after registering all required components; move it around to
    //       satisfy this as needed
    init_test_entities(world, &mut factory);

    // Initialize systems
    let camera = world.read_resource::<Arc<Mutex<camera::Camera>>>().clone();
    let lighting_data = world.read_resource::<Arc<Mutex<lighting_data::LightingData>>>().clone();
    let shadow_source =
        world.read_resource::<Arc<Mutex<passes::shadow::DirShadowSource>>>().clone();

    let draw = super::System::new(
            factory,
            window.clone(),
            device,
            main_color,
            main_depth,
            encoder,
            (point_send, spot_send),
            camera,
            lighting_data,
            shadow_source,
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
    world.register::<components::Drawable<R>>();
}
