//! Types that simplify the building and running of a graphics pipeline

#![deny(missing_docs)]

extern crate common;
extern crate image_utils;

use common::{glutin, shred, gfx, gfx_core};

pub mod pass;
pub mod module;
pub mod builder;
pub mod error;

use shred::{Resources, ResourceId};
use glutin::GlContext;
use gfx::format;

use std::sync::Arc;

/// A type that stores all passes and can run them
pub struct RenderGraph<R, C, D, F, CF, DF>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device,
          F: gfx::Factory<R>,
{
    passes: Vec<Box<pass::Pass<R, C, F>>>,
    resources: Resources,
    encoder: gfx::Encoder<R, C>,
    device: D,
    window: Arc<glutin::GlWindow>,
    main_color: gfx::handle::RenderTargetView<R, CF>,
    main_depth: gfx::handle::DepthStencilView<R, DF>,
}

impl<R, C, D, F, CF, DF> RenderGraph<R, C, D, F, CF, DF>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device<Resources = R, CommandBuffer = C>,
          F: gfx::Factory<R>,
{
    /// Returns a new `RenderGraph` that contains the provided passes and resources
    ///
    /// Requires ownership of the device, and an encoder.
    pub fn new(
        passes: Vec<Box<pass::Pass<R, C, F>>>,
        resources: Resources,
        encoder: gfx::Encoder<R, C>,
        device: D,
        window: Arc<glutin::GlWindow>,
        main_color: gfx::handle::RenderTargetView<R, CF>,
        main_depth: gfx::handle::DepthStencilView<R, DF>,
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
        if self.resources.has_value(ResourceId::new::<Res>()) {
            *self.resources.fetch_mut::<Res>(0) = resource;
        } else {
            self.resources.add(resource);
        }
    }

    /// Executes all passes in the `RenderGraph`
    ///
    /// `RenderGraph::finish` must be called after this to display the results to the window.
    pub fn execute_passes(&mut self) -> Result<(), error::RunError> {
        for pass in &mut self.passes {
            pass.execute_pass(&mut self.encoder, &mut self.resources)?
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
    pub fn reload_shaders(&mut self, factory: &mut F) -> Result<(), error::BuildError<String>> {
        for pass in &mut self.passes {
            pass.reload_shaders(factory)?;
        }
        Ok(())
    }

    /// Returns a mutable reference to the `gfx::Encoder` used by the `RenderGraph`
    pub fn encoder(&mut self) -> &mut gfx::Encoder<R, C> {
        &mut self.encoder
    }
}

impl<R, C, D, F, CF, DF> RenderGraph<R, C, D, F, CF, DF>
    where R: gfx::Resources,
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
