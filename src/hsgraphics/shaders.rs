use gfx::Primitive;
use gfx::PipelineStateError;
use gfx::traits::FactoryExt;
use gfx::state::Rasterizer;
use gfx_device_gl::{Factory, Resources};

use assets::load::load_bytes;
use hsgraphics::{object2d, object3d, gfx2d, gfx3d};

use std::path::Path;

pub fn load_pso2d<P: AsRef<Path>>(factory: &mut Factory,
                                  vs_path: P,
                                  fs_path: P,
                                  primitive: Primitive)

    -> Result<object2d::ObjectPSO, PipelineStateError> {

    let vs = unwrap_pretty!(load_bytes(vs_path));
    let fs = unwrap_pretty!(load_bytes(fs_path));

    let set = try!(factory.create_shader_set(&vs, &fs));

    factory.create_pipeline_state(&set, primitive, Rasterizer::new_fill(), gfx2d::pipe::new())
}

pub fn load_pso3d<P: AsRef<Path>>(factory: &mut Factory,
                                  vs_path: P,
                                  fs_path: P,
                                  primitive: Primitive)
    -> Result<object3d::ObjectPSO, PipelineStateError> {

    let vs = unwrap_pretty!(load_bytes(vs_path));
    let fs = unwrap_pretty!(load_bytes(fs_path));

    let set = try!(factory.create_shader_set(&vs, &fs));

    factory.create_pipeline_state(&set, primitive, Rasterizer::new_fill(), gfx3d::pipe::new())
}
