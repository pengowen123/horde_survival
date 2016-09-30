use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type Color = [f32; 3];

pub const CLEAR_COLOR: [f32; 4] = [0.0, 0.35, 0.5, 1.0];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        color: gfx::TextureSampler<[f32; 4]> = "t_Color",
        out: gfx::RenderTarget<ColorFormat> = "f_Color",
    }
}

impl Vertex {
    pub fn new(pos: [f32; 2], tex_coord: [f32; 2]) -> Vertex {
        Vertex {
            pos: pos,
            tex_coord: tex_coord,
        }
    }
}
