//! Rendering system
//!
//! Draws each entity that has a `Drawable` component and handles displaying the results to the
//! window.

#[macro_use]
mod utils;
pub mod components;
mod passes;
mod param;
mod init;
mod types;
mod factory_ext;
mod lighting_data;
mod render_target;
mod glsl;

pub use self::init::initialize;

// TODO: Remove these re-exports when higher-level functionality is exposed
pub use self::passes::main::geometry_pass::Vertex;
pub use self::passes::main::lighting::Material;
pub use self::passes::shadow::{DirShadowSource, LightSpaceMatrix};
pub use self::types::{ColorFormat, DepthFormat};
pub use self::components::Drawable;
pub use self::param::ShaderParam;

use gfx::{self, handle};
use common::specs;
use rendergraph::{RenderGraph, builder, module, pass};
use window;

use std::sync::{Arc, Mutex, mpsc};

use self::passes::{postprocessing, skybox, resource_pass, shadow};
use self::passes::main::{geometry_pass, lighting};
use self::types::AspectRatio;
use camera;

/// A `specs::Storage` for the `Drawable` component
pub type DrawableStorage<'a, R> =
    specs::Storage<
        'a,
        components::Drawable<R>,
        specs::Fetch<'a, specs::MaskedStorage<components::Drawable<R>>>,
    >;

// NOTE: This should only be passed from draw::System to rendergraph passes, which will always have
//       a smaller lifetime and run on the same thread, and the contained raw pointer will never be
//       null (`None` is used to represent null instead), so this should be safe to use
struct DrawableStorageRef<R: gfx::Resources>(Option<*const DrawableStorage<'static, R>>);

unsafe impl<R: gfx::Resources> Send for DrawableStorageRef<R> {}
unsafe impl<R: gfx::Resources> Sync for DrawableStorageRef<R> {}

impl<R: gfx::Resources> DrawableStorageRef<R> {
    pub fn new<'a>(storage: &'a DrawableStorage<'a, R>) -> Self {
        let storage = storage as *const DrawableStorage<'a, R>;
        let storage: *const DrawableStorage<'static, R> = unsafe {
            ::std::mem::transmute(storage)
        };

        DrawableStorageRef(Some(storage))
    }

    /// Returns a null `DrawableStorageRef`
    pub fn new_null() -> Self {
        DrawableStorageRef(None)
    }

    /// Returns a non-null pointer to the `DrawableStorage`
    pub fn get<'a>(&'a self) -> *const DrawableStorage<'a, R> {
        self.0.unwrap()
    }
}

pub struct System<F, C, R, D>
where
    F: gfx::Factory<R>,
    C: gfx::CommandBuffer<R>,
    R: gfx::Resources,
    D: gfx::Device<Resources = R, CommandBuffer = C>,
{
    factory: F,
    graph: RenderGraph<R, C, D>,
    aspect_ratio_point: mpsc::Sender<AspectRatio>,
    aspect_ratio_spot: mpsc::Sender<AspectRatio>,
}

impl<F, C, R, D> System<F, C, R, D>
where
    F: gfx::Factory<R>,
    C: gfx::CommandBuffer<R>,
    R: gfx::Resources,
    D: gfx::Device<Resources = R, CommandBuffer = C>,
{
    // TODO: Make this return result so the application can handle the error
    pub fn new(
        mut factory: F,
        window: window::Window,
        device: D,
        out_color: handle::RenderTargetView<R, types::ColorFormat>,
        out_depth: handle::DepthStencilView<R, types::DepthFormat>,
        encoder: gfx::Encoder<R, C>,
        (aspect_ratio_point, aspect_ratio_spot):
        (mpsc::Sender<AspectRatio>, mpsc::Sender<AspectRatio>),
        // Shared resources between the `specs::World` and the rendergraph
        camera: Arc<Mutex<camera::Camera>>,
        lighting_data: Arc<Mutex<lighting_data::LightingData>>,
        dir_shadow_source: Arc<Mutex<shadow::DirShadowSource>>,
    ) -> Self {
        // Build the rendergraph
        let graph = {
            let mut builder = builder::GraphBuilder::new(&mut factory, out_color, out_depth);

            builder.add_resource(window::info::WindowInfo::new(window.window()));
            builder.add_resource(camera);
            builder.add_resource(lighting_data);
            builder.add_resource(dir_shadow_source);

            let resource_module = module::Module::new()
                .add_pass(resource_pass::setup_pass::<R, C, F> as pass::SetupFn<_, _, _, _, _>);

            let shadow_module = module::Module::new()
                .add_pass(
                    shadow::directional::setup_pass::<R, C, F> as pass::SetupFn<_, _, _, _, _>
                    );

            let main_module = module::Module::new()
                .add_pass(geometry_pass::setup_pass::<R, C, F> as pass::SetupFn<_, _, _, _, _>)
                .add_pass(lighting::setup_pass::<R, C, F> as pass::SetupFn<_, _, _, _, _>);

            let skybox_module = module::Module::new()
                .add_pass(skybox::setup_pass::<R, C, F> as pass::SetupFn<_, _, _, _, _>);
            
            let postprocessing_module = module::Module::new()
                .add_pass(postprocessing::setup_pass::<R, C, F> as pass::SetupFn<_, _, _, _, _>);

            let modules = vec![
                resource_module,
                shadow_module,
                main_module,
                skybox_module,
                postprocessing_module,
            ];

            for module in modules {
                module.setup_passes(&mut builder);
            }

            builder.build(device, encoder, window)
        };

        Self {
            factory,
            graph,
            aspect_ratio_point,
            aspect_ratio_spot,
        }
    }

    pub fn factory(&self) -> &F {
        &self.factory
    }

    /// Reloads the shaders
    fn reload_shaders(&mut self) -> Result<(), passes::PassError> {
        // TODO
        Ok(())
    }
}

#[derive(SystemData)]
pub struct Data<'a, R: gfx::Resources> {
    drawable: specs::ReadStorage<'a, components::Drawable<R>>,
}

impl<'a, F, C, R, D> specs::System<'a> for System<F, C, R, D>
where
    F: gfx::Factory<R>,
    C: gfx::CommandBuffer<R>,
    R: gfx::Resources,
    D: gfx::Device<Resources = R, CommandBuffer = C>,
{
    type SystemData = Data<'a, R>;

    fn run(&mut self, data: Self::SystemData) {
        // This has the lifetime of this function, and the DrawableStorageRef is set to null before
        // the function ends, so there shouldn't be any dangling pointers
        let drawable: &DrawableStorage<R> = &data.drawable;

        self.graph.add_resource(DrawableStorageRef::new(drawable));

        self.graph.execute_passes();

        self.graph.add_resource(DrawableStorageRef::<R>::new_null())
    }
}
