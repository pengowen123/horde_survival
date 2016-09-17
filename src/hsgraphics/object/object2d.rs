use gfx::{self, Slice};
use gfx::traits::FactoryExt;
use gfx_device_gl::{Factory, Resources};

use hsgraphics::gfx2d::*;
use hsgraphics::object::*;

pub type ObjectPSO = gfx::PipelineState<Resources, pipe::Meta>;

#[derive(Clone)]
pub struct Object2d {
    pub slice: Slice<Resources>,
    pub data: pipe::Data<Resources>,
    pub id: usize,
}

impl Object2d {
    pub fn new(slice: Slice<Resources>, data: pipe::Data<Resources>) -> Object2d {
        Object2d {
            slice: slice,
            data: data,
            // NOTE: id is the category of objects
            //       0 is minimap objects for now
            id: 0,
        }
    }

    pub fn from_slice(factory: &mut Factory,
                            slice: &[Vertex],
                            color: ObjectColor) -> Object2d {
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(slice, ());
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: color,
        };

        Object2d::new(slice, data)
    }
}

impl Object2d {
    pub fn encode(&self, encoder: &mut ObjectEncoder, pso: &ObjectPSO) {
        encoder.draw(&self.slice, pso, &self.data);
    }
}
