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
use common::{self, shred, specs, conrod};
use rendergraph::{RenderGraph, builder, module, pass};
use rendergraph::error::BuildError;
use window::{self, info, window_event};
use slog;
use ui;

use std::sync::{Arc, Mutex};

use self::passes::{postprocessing, skybox, resource_pass, shadow};
use self::passes::main::{geometry_pass, lighting};
use self::lighting_data::LightingData;
use camera::Camera;

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
        self.0.expect("`DrawableStorageRef::get` called on a null pointer")
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
    graph: RenderGraph<R, C, D, F, ColorFormat, DepthFormat>,
    reader_id: window_event::ReaderId,
    ui_renderer: conrod::backend::gfx::Renderer<'static, R>,
}

impl<F, C, R, D> System<F, C, R, D>
where
    F: gfx::Factory<R>,
    C: gfx::CommandBuffer<R>,
    R: gfx::Resources,
    D: gfx::Device<Resources = R, CommandBuffer = C>,
{
    // TODO: Make this return result so the application can handle the error
    pub fn new<'a>(
        mut factory: F,
        window: window::Window,
        device: D,
        out_color: handle::RenderTargetView<R, types::ColorFormat>,
        out_depth: handle::DepthStencilView<R, types::DepthFormat>,
        encoder: gfx::Encoder<R, C>,
        resources: &'a mut shred::Resources,
    ) -> Self {
        // Read resources from the specs World
        let camera = resources.fetch::<Arc<Mutex<Camera>>>(0).clone();
        let lighting_data = resources.fetch::<Arc<Mutex<LightingData>>>(0).clone();
        let dir_shadow_source = resources.fetch::<Arc<Mutex<DirShadowSource>>>(0).clone();
        let reader_id = {
            let mut event_channel = resources.fetch_mut::<window_event::EventChannel>(0);
            event_channel.register_reader()
        };
        let log = resources.fetch::<slog::Logger>(0);

        let dpi = window.get_hidpi_factor();

        // Build the rendergraph
        let graph = {
            let mut builder =
                builder::GraphBuilder::new(&mut factory, out_color.clone(), out_depth);

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
                (resource_module, "resource"),
                (shadow_module, "shadow"),
                (main_module, "main"),
                (skybox_module, "skybox"),
                (postprocessing_module, "postprocessing"),
            ];

            for (module, name) in modules {
                module.setup_passes(&mut builder).unwrap_or_else(|e| {
                    error!(log, "Error setting up `{}` module: {}", name, e);
                    panic!(common::CRASH_MSG);
                });
            }

            builder.build(device, encoder, window)
        };
        
        // Build the UI renderer
        let ui_renderer = conrod::backend::gfx::Renderer::new(
            &mut factory,
            &out_color,
            dpi,
        ).unwrap_or_else(|e| {
            error!(log, "Error building UI renderer: {}", e;);
            panic!(common::CRASH_MSG);
        });

        Self {
            factory,
            graph,
            reader_id,
            ui_renderer,
        }
    }

    pub fn factory(&self) -> &F {
        &self.factory
    }

    /// Reloads the shaders
    fn reload_shaders(&mut self, log: &slog::Logger) -> Result<(), BuildError<String>> {
        info!(log, "Reloading shaders";);
        self.graph.reload_shaders(&mut self.factory)
    }
}

#[derive(SystemData)]
pub struct Data<'a, R: gfx::Resources> {
    drawable: specs::ReadStorage<'a, components::Drawable<R>>,
    event_channel: specs::Fetch<'a, window_event::EventChannel>,
    ui_state: specs::Fetch<'a, common::UiState>,
    ui_draw_list: specs::Fetch<'a, ui::UiDrawList>,
    ui_image_map: specs::Fetch<'a, ui::ImageMap<R>>,
    window_info: specs::Fetch<'a, info::WindowInfo>,
    log: specs::Fetch<'a, slog::Logger>,
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
        // Check if shaders should be reloaded
        for e in data.event_channel.read(&mut self.reader_id) {
            if let &window_event::Event::ReloadShaders = e {
                self.reload_shaders(&data.log)
                    .unwrap_or_else(|e| {
                        error!(data.log, "Error reloading shaders: {}", e;);
                    });
            }
        }

        // This has the lifetime of this function, and the DrawableStorageRef is set to null before
        // the function ends, so there shouldn't be any dangling pointers
        let drawable: &DrawableStorage<R> = &data.drawable;

        self.graph.add_resource(DrawableStorageRef::new(drawable));

        self.graph.clear_targets();

        // Only run the main graphics pipeline if a menu is not open
        if data.ui_state.is_in_game() {
            self.graph.execute_passes().unwrap_or_else(|e| {
                error!(data.log, "Error executing passes: {}", e;);
                panic!(common::CRASH_MSG);
            });
        }

        let (win_w, win_h): (f64, f64) = data.window_info.physical_dimensions().into();
        let image_map = &*data.ui_image_map.get();
        
        if let Some(draw_list) = data.ui_draw_list.walk() {
            self.ui_renderer.fill(
                self.graph.encoder(),
                (win_w as _, win_h as _),
                draw_list,
                image_map,
            );
        } else {
            warn!(data.log, "UI draw list not found";);
        }

        self.ui_renderer.draw(&mut self.factory, self.graph.encoder(), image_map);

        self.graph.finish_frame().unwrap_or_else(|e| {
            error!(data.log, "Error finishing frame: {}", e);
            panic!(common::CRASH_MSG);
        });

        self.graph.add_resource(DrawableStorageRef::<R>::new_null());
    }
}
