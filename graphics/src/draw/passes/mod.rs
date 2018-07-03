//! Render passes

pub mod main;
pub mod postprocessing;
pub mod skybox;
pub mod shadow;
pub mod resource_pass;

use gfx::{self, pso};
use gfx::traits::FactoryExt;
use rendergraph::error::BuildError;

use std::path::Path;

use assets::shader;

/// Loads shaders from the provided paths, and returns a PSO built from the shaders and pipeline
pub fn load_pso<R, F, P, I>(
    factory: &mut F,
    vs_path: P,
    fs_path: P,
    primitive: gfx::Primitive,
    rasterizer: gfx::state::Rasterizer,
    init: I,
) -> Result<pso::PipelineState<R, I::Meta>, BuildError<String>>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    P: AsRef<Path>,
    I: pso::PipelineInit,
{
    let vs = shader::load_shader_file(vs_path)?;
    let fs = shader::load_shader_file(fs_path)?;
    
    // NOTE: If create_pipeline_from_program is used here, the ProgramInfo can be printed, which may
    //       be useful for debugging
    let set = factory.create_shader_set(&vs, &fs)?;

    factory.create_pipeline_state(&set, primitive, rasterizer, init).map_err(|e| e.into())
}
