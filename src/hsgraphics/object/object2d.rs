use gfx::{self, Slice};
use gfx::traits::FactoryExt;
use gfx_device_gl::{Factory, Resources};

use hsgraphics::texture::Texture;
use hsgraphics::gfx2d::*;
use hsgraphics::object::*;

pub type ObjectPSO = gfx::PipelineState<Resources, pipe::Meta>;
pub type VBuffer2d = gfx::handle::Buffer<Resources, Vertex>;

#[derive(Clone)]
pub struct Object2d {
    pub vbuf: VBuffer2d,
    pub slice: Slice<Resources>,
    pub id: usize,
    pub texture: Texture,
}

impl Object2d {
    pub fn new(vbuf: VBuffer2d, slice: Slice<Resources>, texture: Texture) -> Object2d {
        Object2d {
            vbuf: vbuf,
            slice: slice,
            // NOTE: id is the category of objects
            id: 0,
            texture: texture,
        }
    }

    pub fn from_slice(factory: &mut Factory, slice: &[Vertex], texture: Texture) -> Object2d {
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(slice, ());
        Object2d::new(vertex_buffer, slice, texture)
    }

    pub fn from_slice_indices(factory: &mut Factory,
                              slice: &[Vertex],
                              indices: &[u16],
                              texture: Texture)
                              -> Object2d {
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(slice, indices);
        Object2d::new(vertex_buffer, slice, texture)
    }
}

impl Object2d {
    pub fn encode(&self,
                  encoder: &mut ObjectEncoder,
                  pso: &ObjectPSO,
                  data: &mut pipe::Data<Resources>) {
        data.vbuf = self.vbuf.clone();
        data.color.0 = self.texture.clone();
        encoder.draw(&self.slice, pso, data);
    }
}
