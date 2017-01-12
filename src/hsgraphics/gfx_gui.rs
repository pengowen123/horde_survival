pub use hsgraphics::ColorFormat;

/// Types used for gfx
/// GUI vertices contain a position and a color
/// Used for certain parts of the GUI
gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 4] = "a_Color",
    }

    pipeline pipe {
        vbuf: ::gfx::VertexBuffer<Vertex> = (),
        out: ::gfx::BlendTarget<ColorFormat> = ("Target0", ::gfx::state::MASK_ALL, ::gfx::preset::blend::ALPHA),
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
