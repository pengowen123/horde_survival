//! Rendering system
//!
//! Draws each entity that has a `Drawable` component and handles displaying the results to the
//! window.

#[macro_use]
mod utils;
pub mod components;
mod factory_ext;
mod glsl;
mod init;
mod lighting_data;
mod param;
mod passes;
mod render_target;
mod types;

pub use self::init::initialize;

// TODO: Remove these re-exports when higher-level functionality is exposed
pub use self::passes::shadow::{DirShadowSource, LightSpaceMatrix};
pub use self::types::{ColorFormat, DepthFormat};

use assets;
use common::glutin::{self, GlContext};
use common::graphics::{Drawable, ParticleSource};
use common::{self, config, conrod, shred, specs};
use gfx::{self, handle};

use rendergraph::error::Error;
use rendergraph::resources::TemporaryResources;
use rendergraph::{builder, module, pass, RenderGraph};
use slog;
use ui;
use window::{self, info, window_event};

use std::sync::{Arc, Mutex};

use self::lighting_data::LightingData;
use self::passes::main::{geometry_pass, lighting};
use self::passes::{particles, postprocessing, resource_pass, shadow, skybox};
use camera::Camera;

/// A function that creates new window target views
pub type CreateNewWindowViews<R> = Box<
    Fn(
        &glutin::GlWindow
    ) -> (
        handle::RenderTargetView<R, types::ColorFormat>,
        handle::DepthStencilView<R, types::DepthFormat>,
    ),
>;

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
    // NOTE: This field is just a hack to avoid calling `gfx_window_glutin::new_views` in a generic
    //       context. This is necessary because that function returns types that use
    //       `gfx_device_gl::Resources`, which is a concrete type, and the code here attempts to use
    //       its return value where `R`, a type parameter, is expected
    create_new_window_views: CreateNewWindowViews<R>,
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
        create_new_window_views: CreateNewWindowViews<R>,
    ) -> Self {
        // Read resources from the specs World
        let camera = resources.fetch::<Arc<Mutex<Camera>>>().clone();
        let lighting_data = resources.fetch::<Arc<Mutex<LightingData>>>().clone();
        let dir_shadow_source = resources.fetch::<Arc<Mutex<DirShadowSource>>>().clone();
        let reader_id = {
            let mut event_channel = resources.fetch_mut::<window_event::EventChannel>();
            event_channel.register_reader()
        };
        let log = resources.fetch::<slog::Logger>();
        let config = resources.fetch::<config::Config>().graphics.clone();
        let assets = resources.fetch::<Arc<assets::Assets>>();

        let dpi = window.get_hidpi_factor();

        // Build the rendergraph
        let graph = {
            let mut builder =
                builder::GraphBuilder::new(&mut factory, &assets, out_color.clone(), out_depth);

            builder.add_resource(window::info::WindowInfo::new(window.window()));
            builder.add_resource(camera);
            builder.add_resource(lighting_data);
            builder.add_resource(dir_shadow_source);
            builder.add_resource(config);
            builder.add_resource(assets.clone());

            let resource_module = module::Module::new()
                .add_pass(resource_pass::setup_pass::<R, C, F> as pass::SetupFn<_, _, _, _, _>);

            let particles_module = module::Module::new()
                .add_pass(particles::setup_pass::<R, C, F> as pass::SetupFn<_, _, _, _, _>);

            let shadow_module = module::Module::new().add_pass(
                shadow::directional::setup_pass::<R, C, F> as pass::SetupFn<_, _, _, _, _>,
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
                (particles_module, "particles"),
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
        let ui_renderer = conrod::backend::gfx::Renderer::new(&mut factory, &out_color, dpi)
            .unwrap_or_else(|e| {
                error!(log, "Error building UI renderer: {}", e;);
                panic!(common::CRASH_MSG);
            });

        Self {
            factory,
            graph,
            reader_id,
            ui_renderer,
            create_new_window_views,
        }
    }

    pub fn factory(&self) -> &F {
        &self.factory
    }

    /// Reloads the shaders
    fn reload_shaders(
        &mut self,
        assets: &assets::Assets,
        log: &slog::Logger,
    ) -> Result<(), Error<String>> {
        info!(log, "Reloading shaders";);
        self.graph.reload_shaders(&mut self.factory, assets)
    }
}

#[derive(SystemData)]
pub struct Data<'a, R: gfx::Resources> {
    drawable: specs::ReadStorage<'a, Drawable<R>>,
    particle_source: specs::ReadStorage<'a, ParticleSource<R>>,
    event_channel: specs::ReadExpect<'a, window_event::EventChannel>,
    ui_state: specs::ReadExpect<'a, common::UiState>,
    ui_draw_list: specs::ReadExpect<'a, ui::UiDrawList>,
    ui_image_map: specs::ReadExpect<'a, ui::ImageMap<R>>,
    window: specs::ReadExpect<'a, window::Window>,
    window_info: specs::ReadExpect<'a, info::WindowInfo>,
    log: specs::ReadExpect<'a, slog::Logger>,
    config: specs::ReadExpect<'a, config::Config>,
    assets: specs::ReadExpect<'a, Arc<assets::Assets>>,
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
        // Check for relevant window events
        for e in data.event_channel.read(&mut self.reader_id) {
            match *e {
                window_event::Event::ReloadShaders => {
                    self.reload_shaders(&data.assets, &data.log)
                        .unwrap_or_else(|e| {
                            error!(data.log, "Error reloading shaders: {}", e;);
                        });
                }
                window_event::Event::WindowResized(new_size) => {
                    let new_physical_size = new_size.to_physical(data.window.get_hidpi_factor());

                    // Resize the GL context
                    data.window.resize(new_physical_size);

                    let (resized_main_color, resized_main_depth) =
                        (self.create_new_window_views)(self.graph.window());

                    // Handle window resize for render passes
                    self.graph
                        .handle_window_resize(
                            resized_main_color.clone(),
                            resized_main_depth,
                            &mut self.factory,
                        ).unwrap_or_else(|e| {
                            error!(data.log, "Error handling window resize: {}", e;);
                        });

                    // Handle window resize for UI renderer
                    self.ui_renderer.on_resize(resized_main_color);
                }
                window_event::Event::ConfigChanged(window_event::ChangedConfig::Graphics) => {
                    info!(data.log, "Applying graphics configuration changes";);
                    self.graph
                        .apply_config(&data.config.graphics, &mut self.factory, &data.assets)
                        .unwrap_or_else(|e| {
                            error!(data.log, "Error apply graphics configuration: {}", e;);
                        });
                }
                _ => {}
            }
        }

        self.graph.clear_targets();

        // Only run the main graphics pipeline if a menu is not open
        if data.ui_state.is_in_game() {
            let temporary_resources =
                TemporaryResources::new(&data.drawable, &data.particle_source);

            self.graph
                .execute_passes(temporary_resources)
                .unwrap_or_else(|e| {
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

        self.ui_renderer
            .draw(&mut self.factory, self.graph.encoder(), image_map);

        self.graph.finish_frame().unwrap_or_else(|e| {
            error!(data.log, "Error finishing frame: {}", e);
            panic!(common::CRASH_MSG);
        });
    }
}
