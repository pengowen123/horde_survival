use hsgraphics::ColorFormat;

/// The color used when clearing the screen
// TODO: Replace this with [1.0; 4] (white) whena skybox is implemented
//       For now, this color is the sky
pub const CLEAR_COLOR: [f32; 4] = [0.0, 0.35, 0.5, 1.0];

/// Types used for gfx
/// 2d vertices contain a position, a texture coordinate, and a color
/// Used for certain parts of the GUI
gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
        color: [f32; 4] = "a_Color",
    }

    pipeline pipe {
        vbuf: ::gfx::VertexBuffer<Vertex> = (),
        color: ::gfx::TextureSampler<[f32; 4]> = "t_Color",
        out: ::gfx::BlendTarget<ColorFormat> = ("Target0", ::gfx::state::MASK_ALL, ::gfx::preset::blend::ALPHA),
    }
}

impl Vertex {
    pub fn new(pos: [f32; 2], uv: [f32; 2]) -> Vertex {
        Vertex {
            pos: pos,
            uv: uv,
            color: [1.0; 4],
        }
    }

    /// A constructor for colored vertices
    pub fn new_colored(pos: [f32; 2], uv: [f32; 2], color: [f32; 4]) -> Vertex {
        Vertex {
            pos: pos,
            uv: uv,
            color: color,
        }
    }
}
