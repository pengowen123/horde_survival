use gfx::{self, Slice};
use gfx::traits::FactoryExt;
use gfx_device_gl::{Factory, Resources};

use hsgraphics::gfx_gui::{Vertex, pipe};
use hsgraphics::object::ObjectEncoder;

/// PSO for `ObjectGUI`
pub type ObjectPSO = gfx::PipelineState<Resources, pipe::Meta>;
/// Vertex buffer of 2d vertices for the GUI
pub type VBufferGUI = gfx::handle::Buffer<Resources, Vertex>;

/// A 2d object for the GUI
pub struct ObjectGUI {
    pub vbuf: VBufferGUI,
    pub slice: Slice<Resources>,
}

impl ObjectGUI {
    /// Creates an ObjectGUI from a list of vertices, a texture, an index buffer
    pub fn new(factory: &mut Factory, vertices: &[Vertex], indices: &[u16]) -> ObjectGUI {
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);

        ObjectGUI {
            vbuf: vbuf,
            slice: slice,
        }
    }

    /// Draws the object
    pub fn encode(&self,
                  encoder: &mut ObjectEncoder,
                  pso: &ObjectPSO,
                  data: &mut pipe::Data<Resources>) {
        data.vbuf = self.vbuf.clone();
        encoder.draw(&self.slice, pso, data);
    }
}
