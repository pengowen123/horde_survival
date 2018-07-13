//! This pass handles resources such as intermediate targets that are used by other passes

use gfx::{self, handle, format, texture};
use window::info::WindowInfo;
use rendergraph::pass::Pass;
use rendergraph::error::{RunError, BuildError};
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
    pub depth_srv: handle::ShaderResourceView<R, [f32; 4]>,
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
        let (_, srv, rtv) = factory
            .create_render_target(dim.0, dim.1)?;
        let (_, depth_srv, dsv) = factory.create_depth_stencil(dim.0, dim.1)?;
        
        IntermediateTarget {
            rtv,
            srv,
            dsv,
            depth_srv,
        }
    };

    let pass = ResourcePass{
        intermediate_target: intermediate_target.clone()
    };
    
    builder.add_pass(pass);
    builder.add_pass_output("intermediate_target", intermediate_target);

    Ok(())
}

pub struct ResourcePass<R: gfx::Resources> {
    intermediate_target: IntermediateTarget<R>,
}

impl<R, C, F> Pass<R, C, F> for ResourcePass<R>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    fn execute_pass(&mut self, encoder: &mut gfx::Encoder<R, C>, _: &mut Resources)
        -> Result<(), RunError>
    {
        encoder.clear(&self.intermediate_target.rtv, [0.0; 4]);
        encoder.clear_depth(&self.intermediate_target.dsv, 1.0);
        
        Ok(())
    }

    fn reload_shaders(&mut self, _: &mut F) -> Result<(), BuildError<String>> {
        Ok(())
    }
}
