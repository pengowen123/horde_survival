//! Render passes

pub mod main;
pub mod postprocessing;
pub mod skybox;
pub mod shadow;
pub mod resource_pass;

use gfx::{self, pso};
use gfx::traits::FactoryExt;
use image_utils;
use rendergraph;

use std::path::Path;
use std::io;

use assets::shader;

/// Loads shaders from the provided paths, and returns a PSO built from the shaders and pipeline
pub fn load_pso<R, F, P, I>(
    factory: &mut F,
    vs_path: P,
    fs_path: P,
    primitive: gfx::Primitive,
    rasterizer: gfx::state::Rasterizer,
    init: I,
) -> Result<pso::PipelineState<R, I::Meta>, PassError>
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

    factory
        .create_pipeline_state(&set, primitive, rasterizer, init)
        .map_err(|e| e.into())
}

/// Like `load_pso`, but also loads a geometry shader
pub fn load_pso_geometry<R, F, P, I>(
    factory: &mut F,
    vs_path: P,
    gs_path: P,
    fs_path: P,
    primitive: gfx::Primitive,
    rasterizer: gfx::state::Rasterizer,
    init: I,
) -> Result<pso::PipelineState<R, I::Meta>, PassError>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    P: AsRef<Path>,
    I: pso::PipelineInit,
{
    let vs = shader::load_shader_file(vs_path)?;
    let gs = shader::load_shader_file(gs_path)?;
    let fs = shader::load_shader_file(fs_path)?;
    let set = factory.create_shader_set_geometry(&vs, &gs, &fs)?;

    factory
        .create_pipeline_state(&set, primitive, rasterizer, init)
        .map_err(|e| e.into())
}

quick_error! {
    /// An error while creating a pass
    #[derive(Debug)]
    pub enum PassError {
        Io(err: io::Error) {
            display("Io error: {}", err)
            from()
        }
        PipelineState(err: gfx::PipelineStateError<String>) {
            display("Pipeline state error: {}", err)
            from()
        }
        ProgramError(err: gfx::shade::ProgramError) {
            display("Program error: {}", err)
            from()
        }
        Texture(err: image_utils::TextureError) {
            display("Texture creation error: {}", err)
            from()
        }
        GfxCombined(err: gfx::CombinedError) {
            display("gfx error: {}", err)
            from()
        }
        ShaderLoadingError(err: shader::ShaderLoadingError) {
            display("Shader error: {}", err)
            from()
        }
        PassOutput(err: rendergraph::builder::PassOutputError)
    }
}
