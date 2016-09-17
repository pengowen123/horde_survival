use gfx::{self, Slice};
use gfx::format::Rgba8;
use gfx::traits::FactoryExt;
use gfx_device_gl::{Resources, Factory};

use hsgraphics::*;

pub type ObjectPSO = gfx::PipelineState<Resources, gfx3d::pipe::Meta>;
pub type ShaderView = gfx::handle::ShaderResourceView<Resources, [f32; 4]>;

pub struct Object3d {
    slice: Slice<Resources>,
    data: gfx3d::pipe::Data<Resources>,
    pub id: usize,
}

impl Object3d {
    pub fn new(slice: Slice<Resources>, data: gfx3d::pipe::Data<Resources>) -> Object3d {
        Object3d {
            slice: slice,
            data: data,
            id: 0,
        }
    }

    pub fn from_slice(factory: &mut Factory,
                      (slice, index_data): (Vec<gfx3d::Vertex>, Vec<u16>),
                      color: gfx::handle::RenderTargetView<Resources, Rgba8>,
                      depth: gfx::handle::DepthStencilView<Resources, ObjectDepth>,
                      texture: ShaderView,
                      sampler: gfx::handle::Sampler<Resources>) -> Object3d
    {

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&slice, index_data.as_slice());
        let data = gfx3d::pipe::Data {
                vbuf: vbuf,
                transform: [[0.0; 4]; 4],
                locals: factory.create_constant_buffer(1),
                color: (texture, sampler),
                out_color: color,
                out_depth: depth,
        };

        Object3d::new(slice, data)
    }
}

impl Object3d {
    pub fn encode(&self, encoder: &mut ObjectEncoder, pso: &ObjectPSO, transform: [[f32; 4]; 4]) {
        let locals = gfx3d::Locals { transform: transform };
        encoder.update_constant_buffer(&self.data.locals, &locals);
        encoder.draw(&self.slice, pso, &self.data);
    }
}
