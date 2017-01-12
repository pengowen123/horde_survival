use gfx;
use gfx::traits::FactoryExt;
use gfx_device_gl::{Resources, Factory};

use hsgraphics::*;

pub type Object3dColor = gfx::handle::RenderTargetView<Resources, ColorFormat>;
pub type Object3dDepth = gfx::handle::DepthStencilView<Resources, ObjectDepth>;
/// PSO for Object3d
pub type ObjectPSO = gfx::PipelineState<Resources, gfx3d::pipe::Meta>;
/// Vertex buffer of 3d vertices
pub type VBuffer = gfx::handle::Buffer<Resources, gfx3d::Vertex>;

/// A 3d object
#[derive(Clone)]
pub struct Object3d {
    pub id: usize,
    slice: gfx::Slice<Resources>,
    buf: VBuffer,
    texture: Texture,
}

impl Object3d {
    pub fn new(slice: gfx::Slice<Resources>, buf: VBuffer, texture: Texture) -> Object3d {
        Object3d {
            id: 0,
            slice: slice,
            buf: buf,
            texture: texture,
        }
    }

    /// Creates an Object3d from a list of vertices, a texture, an index buffer
    pub fn from_slice(factory: &mut Factory,
                      slice: &[gfx3d::Vertex],
                      index_data: &[u16],
                      texture: Texture)
                      -> Object3d {
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(slice, index_data);
        Object3d::new(slice, vbuf, texture)
    }
}

impl Object3d {
    /// Draws the object
    pub fn encode(&self,
                  encoder: &mut ObjectEncoder,
                  pso: &ObjectPSO,
                  data: &mut gfx3d::pipe::Data<Resources>) {
        data.color.0 = self.texture.clone();
        data.vbuf = self.buf.clone();
        encoder.draw(&self.slice, pso, data);
    }
}
