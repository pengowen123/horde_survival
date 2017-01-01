use gfx::pso::PipelineInit;
use gfx::traits::FactoryExt;
use gfx::state::Rasterizer;
use gfx::{PipelineState, Primitive, PipelineStateError};
use gfx_device_gl::{Factory, Resources, Device};
use shader_version::glsl::GLSL;
use shader_version::{Shaders, PickShader};

use assets::load::load_bytes;

use std::path::Path;

pub fn load_pso<P, I>(factory: &mut Factory,
                      vs_path: P,
                      fs_path: P,
                      primitive: Primitive,
                      pipe: I)
                      -> Result<PipelineState<Resources, I::Meta>, PipelineStateError>
    where I: PipelineInit,
          P: AsRef<Path> + Clone
{
    let vs = match load_bytes(vs_path.clone()) {
        Ok(b) => b,
        Err(e) => {
            crash!("{}",
                   format!("Failed to load vertex shader source ({}): {}",
                           vs_path.as_ref().to_str().unwrap(),
                           e))
        }
    };

    let fs = match load_bytes(fs_path.clone()) {
        Ok(b) => b,
        Err(e) => {
            crash!("{}",
                   format!("Failed to load fragment shader source ({}): {}",
                           fs_path.as_ref().to_str().unwrap(),
                           e))
        }
    };

    let set = try!(factory.create_shader_set(&vs, &fs));

    factory.create_pipeline_state(&set, primitive, Rasterizer::new_fill(), pipe)
}

pub fn get_shader_version_path(device: &Device, shader_assets_path: &str, suffix: &str) -> String {
    let dev_glsl_version = device.get_info().shading_language;
    let glsl_version = match (dev_glsl_version.major, dev_glsl_version.minor) {
        (1, 10) => GLSL::V1_10,
        (1, 20) => GLSL::V1_20,
        (1, 30) => GLSL::V1_30,
        (1, 40) => GLSL::V1_40,
        (1, 50) => GLSL::V1_50,
        (3, 30) => GLSL::V3_30,
        (4, 00) => GLSL::V4_00,
        (4, 10) => GLSL::V4_10,
        (4, 20) => GLSL::V4_20,
        (4, 30) => GLSL::V4_30,
        (4, 40) => GLSL::V4_40,
        (4, 50) => GLSL::V4_50,
        v => panic!("Unknown GLSL version: {:?}", v),
    };

    let mut shaders = Shaders::new();

    shaders.set(GLSL::V1_20, "120")
        .set(GLSL::V1_50, "150");

    let version = glsl_version.pick_shader(&shaders)
        .expect(&format!("Failed to pick shader (GLSL {:?}", dev_glsl_version));

    format!("{}/{}/{}", shader_assets_path, version, suffix)
}
