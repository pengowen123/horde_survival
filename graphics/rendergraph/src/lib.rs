extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate cgmath;
extern crate shred;

pub mod pass;
pub mod module;
pub mod builder;

use shred::Resources;
use glutin::GlContext;

use std::sync::Arc;

pub struct RenderGraph<R, C, D>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device,
{
    passes: Vec<Box<pass::Pass<R, C>>>,
    resources: Resources,
    encoder: gfx::Encoder<R, C>,
    device: D,
    window: Arc<glutin::GlWindow>,
}

impl<R, C, D> RenderGraph<R, C, D>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          D: gfx::Device<Resources = R, CommandBuffer = C>,
{
    pub fn new(passes: Vec<Box<pass::Pass<R, C>>>,
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

    pub fn add_resource<Res: shred::Resource>(&mut self, resource: Res) {
        self.resources.add(resource);
    }

    pub fn execute_passes(&mut self) {
        for pass in &mut self.passes {
            pass.execute_pass(&mut self.encoder, &mut self.resources)
        }

        self.encoder.flush(&mut self.device);
        self.window.swap_buffers().expect("Failed to swap buffers");
        self.device.cleanup();
    }
}
