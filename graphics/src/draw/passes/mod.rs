//! Render passes

pub mod main;
pub mod postprocessing;
pub mod resource_pass;
pub mod shadow;
pub mod skybox;

use assets;
use gfx::traits::FactoryExt;
use gfx::{self, pso};
use rendergraph::error::BuildError;

use std::collections::HashMap;
use std::path::Path;

use assets::shader;

/// Loads shaders from the provided paths, and returns a PSO built from the shaders and pipeline
pub fn load_pso<R, F, P, I>(
    assets: &assets::Assets,
    factory: &mut F,
    vs_path: P,
    fs_path: P,
    primitive: gfx::Primitive,
    rasterizer: gfx::state::Rasterizer,
    init: I,
    defines: HashMap<String, String>,
) -> Result<pso::PipelineState<R, I::Meta>, BuildError<String>>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    P: AsRef<Path>,
    I: pso::PipelineInit,
{
    let vs = shader::load_shader_file(assets, vs_path, &defines)
        .map_err(|e| BuildError::Custom(e.into()))?;
    let fs = shader::load_shader_file(assets, fs_path, &defines)
        .map_err(|e| BuildError::Custom(e.into()))?;

    // NOTE: If create_pipeline_from_program is used here, the ProgramInfo can be printed, which may
    //       be useful for debugging
    let set = factory.create_shader_set(&vs, &fs)?;

    factory
        .create_pipeline_state(&set, primitive, rasterizer, init)
        .map_err(|e| e.into())
}
