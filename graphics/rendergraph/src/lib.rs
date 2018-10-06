//! Types that simplify the building and running of a graphics pipeline

#![deny(missing_docs)]

extern crate assets;
extern crate common;
extern crate image_utils;

use common::{config, gfx, gfx_core, glutin, shred};

pub mod builder;
pub mod error;
pub mod framebuffer;
pub mod module;
pub mod pass;
pub mod resources;

use gfx::{format, handle};
use glutin::GlContext;
use shred::Resources;

use std::sync::Arc;

/// A type that stores all passes and can run them
pub struct RenderGraph<R, C, D, F, CF, DF>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    D: gfx::Device,
    F: gfx::Factory<R>,
{
    passes: Vec<Box<pass::Pass<R, C, F, CF, DF>>>,
    resources: Resources,
    encoder: gfx::Encoder<R, C>,
    device: D,
    window: Arc<glutin::GlWindow>,
    main_color: handle::RenderTargetView<R, CF>,
    main_depth: handle::DepthStencilView<R, DF>,
}

impl<R, C, D, F, CF, DF> RenderGraph<R, C, D, F, CF, DF>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    D: gfx::Device<Resources = R, CommandBuffer = C>,
    F: gfx::Factory<R>,
{
    /// Returns a new `RenderGraph` that contains the provided passes and resources
    ///
    /// Requires ownership of the device, and an encoder.
    pub fn new(
        passes: Vec<Box<pass::Pass<R, C, F, CF, DF>>>,
        resources: Resources,
        encoder: gfx::Encoder<R, C>,
        device: D,
        window: Arc<glutin::GlWindow>,
        main_color: handle::RenderTargetView<R, CF>,
        main_depth: handle::DepthStencilView<R, DF>,
    ) -> Self {
        Self {
            passes,
            resources,
            encoder,
            device,
            window,
            main_color,
            main_depth,
        }
    }

    /// Adds a resource of any type to the `RenderGraph`, making it available to passes
    pub fn add_resource<Res: shred::Resource>(&mut self, resource: Res) {
        self.resources.insert(resource);
    }

    /// Executes all passes in the `RenderGraph`
    ///
    /// `RenderGraph::finish` must be called after this to display the results to the window.
    pub fn execute_passes(
        &mut self,
        temporary_resources: resources::TemporaryResources<R>,
    ) -> Result<(), error::Error<String>> {
        for pass in &mut self.passes {
            pass.execute_pass(&mut self.encoder, &mut self.resources, temporary_resources)
                .map_err(|e| error::Error::new(pass.name().to_string(), error::ErrorKind::Run(e)))?
        }

        Ok(())
    }

    /// Runs end-of-frame code (flush commands to device, swap window buffers, cleanup resources)
    pub fn finish_frame(&mut self) -> Result<(), error::RunError> {
        self.encoder.flush(&mut self.device);
        self.window.swap_buffers()?;
        self.device.cleanup();

        Ok(())
    }

    /// Reloads the shaders for all passes in the `RenderGraph`
    pub fn reload_shaders(
        &mut self,
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<(), error::Error<String>> {
        for pass in &mut self.passes {
            pass.reload_shaders(factory, assets).map_err(|e| {
                error::Error::new(pass.name().to_string(), error::ErrorKind::Build(e))
            })?;
        }
        Ok(())
    }

    /// Handles the window being resized for all passes in the `RenderGraph`
    pub fn handle_window_resize(
        &mut self,
        resized_main_color: handle::RenderTargetView<R, CF>,
        resized_main_depth: handle::DepthStencilView<R, DF>,
        factory: &mut F,
    ) -> Result<(), error::Error<String>> {
        let new_dimensions = resized_main_color.get_dimensions();
        let new_dimensions = (new_dimensions.0, new_dimensions.1);

        self.main_color = resized_main_color;
        self.main_depth = resized_main_depth;

        let mut framebuffers =
            framebuffer::Framebuffers::new(self.main_color.clone(), self.main_depth.clone());

        for pass in &mut self.passes {
            pass.handle_window_resize(new_dimensions, &mut framebuffers, factory)
                .map_err(|e| {
                    error::Error::new(pass.name().to_string(), error::ErrorKind::Build(e))
                })?;
        }
        Ok(())
    }

    /// Applies the provided `Config` to the `RenderGraph`
    pub fn apply_config(
        &mut self,
        config: &config::GraphicsConfig,
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<(), error::Error<String>> {
        let mut framebuffers =
            framebuffer::Framebuffers::new(self.main_color.clone(), self.main_depth.clone());

        for pass in &mut self.passes {
            pass.apply_config(config, &mut framebuffers, factory, assets)
                .map_err(|e| {
                    error::Error::new(pass.name().to_string(), error::ErrorKind::Build(e))
                })?;
        }

        Ok(())
    }

    /// Returns a reference to the window used by the `RenderGraph`
    pub fn window(&self) -> &glutin::GlWindow {
        &*self.window
    }
    /// Returns a mutable reference to the `gfx::Encoder` used by the `RenderGraph`
    pub fn encoder(&mut self) -> &mut gfx::Encoder<R, C> {
        &mut self.encoder
    }
}

impl<R, C, D, F, CF, DF> RenderGraph<R, C, D, F, CF, DF>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    D: gfx::Device<Resources = R, CommandBuffer = C>,
    F: gfx::Factory<R>,
    CF: format::Formatted,
    CF::Surface: format::RenderSurface,
    CF::Channel: format::RenderChannel,
    CF::View: Default,
    gfx_core::command::ClearColor: From<CF::View>,
    DF: format::Formatted,
    DF::Surface: format::DepthSurface,
    DF::Channel: format::RenderChannel,
{
    /// Clears the main color and depth targets
    ///
    /// This should be called before `RenderGraph::execute_passes`.
    pub fn clear_targets(&mut self) {
        self.encoder.clear(&self.main_color, CF::View::default());
        self.encoder.clear_depth(&self.main_depth, 1.0);
    }
}
