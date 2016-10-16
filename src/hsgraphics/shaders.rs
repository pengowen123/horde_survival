use gfx::pso::{PipelineData, PipelineInit};
use gfx::traits::FactoryExt;
use gfx::state::Rasterizer;
use gfx::{PipelineState, Primitive, PipelineStateError};
use gfx_device_gl::{Factory, Resources};

use assets::load::load_bytes;
use hsgraphics::*;

use std::path::Path;

pub fn load_pso<P, I>(factory: &mut Factory,
                      vs_path: P,
                      fs_path: P,
                      primitive: Primitive,
                      pipe: I) -> Result<PipelineState<Resources, I::Meta>, PipelineStateError>

    where I: PipelineInit, P: AsRef<Path>
{
    let vs = unwrap_pretty!(load_bytes(vs_path));
    let fs = unwrap_pretty!(load_bytes(fs_path));

    let set = try!(factory.create_shader_set(&vs, &fs));

    factory.create_pipeline_state(&set, primitive, Rasterizer::new_fill(), pipe)
}
