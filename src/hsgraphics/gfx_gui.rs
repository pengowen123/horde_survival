pub use gfx::{self, Slice};
use gfx::traits::FactoryExt;
use gfx_device_gl::{Factory, Resources};

pub use hsgraphics::ColorFormat;
use hsgraphics::object::ObjectEncoder;

pub type Pso = gfx::PipelineState<Resources, pipe::Meta>;
pub type VBufferGUI = gfx::handle::Buffer<Resources, Vertex>;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::MASK_ALL, gfx::preset::blend::ALPHA),
    }
}

impl Vertex {
    pub fn new(pos: [f32; 2], color: [f32; 4]) -> Vertex {
        Vertex {
            pos: pos,
            color: color,
        }
    }
}

pub struct GUIObject {
    pub vbuf: VBufferGUI,
    pub slice: Slice<Resources>,
}

impl GUIObject {
    pub fn new(factory: &mut Factory, vertices: &[Vertex], indices: &[u16]) -> GUIObject {
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(vertices, indices);

        GUIObject {
            vbuf: vbuf,
            slice: slice,
        }
    }

    pub fn encode(&self, encoder: &mut ObjectEncoder, pso: &Pso, data: &mut pipe::Data<Resources>) {
        data.vbuf = self.vbuf.clone();
        encoder.draw(&self.slice, pso, data);
    }
}
