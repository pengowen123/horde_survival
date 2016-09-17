use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type Color = [f32; 3];

pub const CLEAR_COLOR: [f32; 4] = [0.0, 0.35, 0.5, 1.0];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: Color = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}
