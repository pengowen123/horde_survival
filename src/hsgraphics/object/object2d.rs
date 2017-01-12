use gfx::{self, Slice, IntoIndexBuffer};
use gfx::traits::FactoryExt;
use gfx_device_gl::{Factory, Resources};

use hsgraphics::texture::Texture;
use hsgraphics::gfx2d::*;
use hsgraphics::object::*;

/// PSO for Object2d
pub type ObjectPSO = gfx::PipelineState<Resources, pipe::Meta>;
/// Vertex buffer of 2d vertices
pub type VBuffer2d = gfx::handle::Buffer<Resources, Vertex>;

/// A 2d object
#[derive(Clone)]
pub struct Object2d {
    pub vbuf: VBuffer2d,
    pub slice: Slice<Resources>,
    // TODO: Replace this with an enum and replace ID constants with enum values, to make IDs
    //       strongly typed
    // NOTE: id is the category of objects
    pub id: usize,
    pub texture: Texture,
}

impl Object2d {
    /// Creates an Object2d from a list of vertices, a texture, and something that can be converted
    /// into an index buffer
    /// To use no index buffer, pass `()` instead
    pub fn new<B>(factory: &mut Factory, slice: &[Vertex], texture: Texture, indices: B) -> Object2d
        where B: IntoIndexBuffer<Resources>
    {
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(slice, indices);

        Object2d {
            vbuf: vertex_buffer,
            slice: slice,
            id: 0,
            texture: texture,
        }
    }
}

impl Object2d {
    /// Draws the object
    pub fn encode(&self,
                  encoder: &mut ObjectEncoder,
                  pso: &ObjectPSO,
                  data: &mut pipe::Data<Resources>) {
        data.vbuf = self.vbuf.clone();
        data.color.0 = self.texture.clone();
        encoder.draw(&self.slice, pso, data);
    }
}
