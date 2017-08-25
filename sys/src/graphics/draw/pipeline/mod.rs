//! Graphics pipeline declaration and creation

// TODO: List here a graph of the whole graphics pipeline

pub mod main;
pub mod postprocessing;
pub mod skybox;
pub mod dir_shadow;

use gfx::{self, pso};
use gfx::traits::FactoryExt;
use image_utils;

use std::path::Path;
use std::io;

use assets::shader;

/// A GLSL `vec2`
pub type Vec2 = [f32; 2];
/// A GLSL `vec3`
pub type Vec3 = [f32; 3];
/// A GLSL `vec4`
pub type Vec4 = [f32; 4];
/// A GLSL `mat4`
pub type Mat4 = [Vec4; 4];

/// A PSO and its `Data` struct
pub struct Pipeline<R, D>
where
    R: gfx::Resources,
    D: pso::PipelineData<R>,
{
    pub pso: gfx::PipelineState<R, D::Meta>,
    pub data: D,
}

impl<R, D> Pipeline<R, D>
where
    R: gfx::Resources,
    D: gfx::pso::PipelineData<R>,
{
    pub fn new(pso: pso::PipelineState<R, D::Meta>, data: D) -> Self {
        Self { pso, data }
    }
}

/// Loads shaders from the provided paths, and returns a PSO built from the shaders and pipeline
pub fn load_pso<R, F, P, I>(
    factory: &mut F,
    vs_path: P,
    fs_path: P,
    primitive: gfx::Primitive,
    rasterizer: gfx::state::Rasterizer,
    init: I,
) -> Result<pso::PipelineState<R, I::Meta>, PipelineError>
where
    R: gfx::Resources,
    F: gfx::Factory<R>,
    P: AsRef<Path>,
    I: pso::PipelineInit,
{
    let vs = shader::load_shader_file(vs_path)?;
    let fs = shader::load_shader_file(fs_path)?;
    let set = factory.create_shader_set(&vs, &fs)?;

    factory
        .create_pipeline_state(&set, primitive, rasterizer, init)
        .map_err(|e| e.into())
}

quick_error! {
    /// An error while creating a pipeline
    #[derive(Debug)]
    pub enum PipelineError {
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
    }
}
