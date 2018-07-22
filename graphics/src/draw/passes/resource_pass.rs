//! This pass handles resources such as intermediate targets that are used by other passes

use gfx::{self, handle, format, texture};
use window::info::WindowInfo;
use rendergraph::pass::Pass;
use rendergraph::framebuffer::Framebuffers;
use rendergraph::error::{RunError, BuildError};
use common::config;
use shred::Resources;

use draw::types;

/// An intermediate render and depth target for all passes to use
///
/// The postprocessing pass will read from this target
#[derive(Clone)]
pub struct IntermediateTarget<R: gfx::Resources> {
    pub rtv: handle::RenderTargetView<R, format::Rgba8>,
    pub srv: handle::ShaderResourceView<R, [f32; 4]>,
    pub dsv: handle::DepthStencilView<R, types::DepthFormat>,
}

impl<R: gfx::Resources> IntermediateTarget<R> {
    fn new<F: gfx::Factory<R>>(factory: &mut F, dimensions: (texture::Size, texture::Size))
        -> Result<Self, BuildError<String>>
    {
        let (_, srv, rtv) = factory
            .create_render_target(dimensions.0, dimensions.1)?;
        let (_, _, dsv) = factory.create_depth_stencil(dimensions.0, dimensions.1)?;
        
        Ok(IntermediateTarget {
            rtv,
            srv,
            dsv,
        })
    }
}

pub fn setup_pass<R, C, F>(builder: &mut types::GraphBuilder<R, C, F>)
    -> Result<(), BuildError<String>>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    let intermediate_target = {
        let dim: (u32, u32) = builder
            .get_resources()
            .fetch::<WindowInfo>(0)
            .physical_dimensions()
            .into();
        let factory = builder.factory();
        let dim = (dim.0 as texture::Size, dim.1 as texture::Size);
        IntermediateTarget::new(factory, dim)?
    };

    let pass = ResourcePass {
        intermediate_target: intermediate_target.clone(),
        postprocessing: true,
    };
    
    builder.add_pass(pass);
    builder.add_pass_output("intermediate_target", intermediate_target);

    Ok(())
}

pub struct ResourcePass<R: gfx::Resources> {
    intermediate_target: IntermediateTarget<R>,
    postprocessing: bool,
}

impl<R, C, F> Pass<R, C, F, types::ColorFormat, types::DepthFormat> for ResourcePass<R>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    fn name(&self) -> &str {
        "resource"
    }

    fn execute_pass(&mut self, encoder: &mut gfx::Encoder<R, C>, _: &mut Resources)
        -> Result<(), RunError>
    {
        // The intermediate target is not used when postprocessing is disabled
        if !self.postprocessing {
            return Ok(());
        }

        encoder.clear(&self.intermediate_target.rtv, [0.0; 4]);
        encoder.clear_depth(&self.intermediate_target.dsv, 1.0);
        
        Ok(())
    }

    fn reload_shaders(&mut self, _: &mut F) -> Result<(), BuildError<String>> {
        Ok(())
    }

    fn handle_window_resize(
        &mut self,
        new_dimensions: (u16, u16),
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        factory: &mut F,
    ) -> Result<(), BuildError<String>> {
        if !self.postprocessing {
            return Ok(());
        }

        // Build new intermediate targets using the new window dimensions
        let dim = (new_dimensions.0 as texture::Size, new_dimensions.1 as texture::Size);

        let intermediate_target = IntermediateTarget::new(factory, dim)?;

        // Update the framebuffer that gets cleared by the resource pass
        self.intermediate_target = intermediate_target.clone();

        framebuffers.add_framebuffer("intermediate_target", intermediate_target);

        Ok(())
    }

    fn apply_config(
        &mut self,
        config: &config::GraphicsConfig,
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        factory: &mut F,
    ) -> Result<(), BuildError<String>> {
        // If the postprocessing setting was disabled, set the intermediate targets to dummy
        // textures to save memory
        // NOTE: All references to the old intermediate targets must be dropped, or their memory
        //       won't be freed, making this optimization useless
        // TODO: Test that this is the case
        if !config.postprocessing && self.postprocessing {
            println!("resource pass: disabling postprocessing");
            self.intermediate_target = IntermediateTarget::new(factory, (1, 1))?;
        }

        // If the postprocessing setting was enabled, make new intermediate targets
        if config.postprocessing && !self.postprocessing {
            println!("resource pass: enabling postprocessing");
            // Use the width and height of the main color target (should be the window size)
            let (w, h, _, _) = framebuffers.get_main_color().get_dimensions();
            println!("main target width, height: {:?}", (w, h));
            let intermediate_target = IntermediateTarget::new(factory, (w, h))?;

            self.intermediate_target = intermediate_target.clone();

            framebuffers.add_framebuffer("intermediate_target", intermediate_target);
        }

        self.postprocessing = config.postprocessing;

        Ok(())
    }
}
