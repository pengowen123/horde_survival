use {gfx, gfx_device_gl};
use hsgraphics::gfx_utils::*;

use gfx::{format, Slice};
use gfx::traits::FactoryExt;
use gfx_device_gl::{Resources, Factory};

pub type ObjectEncoder = gfx::Encoder<Resources, gfx_device_gl::CommandBuffer>;
pub type ObjectPSO = gfx::PipelineState<Resources, pipe::Meta>;
pub type ObjectColor = gfx::handle::RenderTargetView<Resources, (format::R8_G8_B8_A8, format::Unorm)>;

#[derive(Clone)]
pub struct Object {
    pub slice: Slice<Resources>,
    pub data: pipe::Data<Resources>,
    pub id: usize,
    pub pso_id: usize,
}

impl Object {
    pub fn new(pso_id: usize, slice: Slice<Resources>, data: pipe::Data<Resources>) -> Object {
        Object {
            slice: slice,
            data: data,
            // NOTE: id is the category of objects
            //       0 is minimap objects for now
            id: 0,
            pso_id: pso_id,
        }
    }

    pub fn from_slice(pso_id: usize,
                      factory: &mut Factory,
                      slice: &[Vertex],
                      color: ObjectColor) -> Object {

        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(slice, ());
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: color,
        };

        Object::new(pso_id, slice, data)
    }
}

impl Object {
    pub fn encode(&self, encoder: &mut ObjectEncoder, pso: &ObjectPSO) {
        encoder.draw(&self.slice, pso, &self.data);
    }
}
