//! Pipeline declaration for postprocessing

use gfx;

use super::*;

/// A `Pipeline` for the postprocessing shaders
pub type Pipeline<R> = super::Pipeline<R, pipe::Data<R>>;

gfx_defines! {
    vertex Vertex {
        pos: Vec2 = "a_Pos",
        uv: Vec2 = "a_Uv",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<Vec4> = "t_Screen",
        screen_color: gfx::RenderTarget<ColorFormat> = "Target0",
   }
}

impl Vertex {
    pub fn new(pos: Vec2, uv: Vec2) -> Self {
        Self { pos, uv }
    }
}
