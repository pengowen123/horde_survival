//! Types that simplify the building and running of a graphics pipeline

#![deny(missing_docs)]

extern crate common;
extern crate image_utils;

use common::{glutin, shred, gfx};

pub mod pass;
pub mod module;
pub mod builder;
pub mod error;

use shred::{Resources, ResourceId};
use glutin::GlContext;

use std::sync::Arc;

/// A type that stores all passes and can run them
pub struct RenderGraph<R, C, D, F>
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
}

impl<R, C, D, F> RenderGraph<R, C, D, F>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device<Resources = R, CommandBuffer = C>,
          F: gfx::Factory<R>,
{
    /// Returns a new `RenderGraph` that contains the provided passes and resources
    ///
    /// Requires ownership of the device, and an encoder.
    pub fn new(passes: Vec<Box<pass::Pass<R, C, F>>>,
               resources: Resources,
               encoder: gfx::Encoder<R, C>,
               device: D,
               window: Arc<glutin::GlWindow>,
    ) -> Self {
        Self {
            passes,
            resources,
            encoder,
            device,
            window,
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
    pub fn execute_passes(&mut self) -> Result<(), error::RunError> {
        self.device.cleanup();

        for pass in &mut self.passes {
            pass.execute_pass(&mut self.encoder, &mut self.resources)?
        }

        self.encoder.flush(&mut self.device);
        self.window.swap_buffers()?;

        Ok(())
    }

    /// Reloads the shaders for all passes in the `RenderGraph`
    pub fn reload_shaders(&mut self, factory: &mut F) -> Result<(), error::BuildError<String>> {
        for pass in &mut self.passes {
            pass.reload_shaders(factory)?;
        }
        Ok(())
    }
}
