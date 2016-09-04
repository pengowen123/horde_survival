use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type ColorDepth = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub const SQUARE: [Vertex; 4] = [
    Vertex { pos: [0.5, 0.5], color: [0.5, 0.5, 0.5] },
    Vertex { pos: [-0.5, 0.5], color: [0.5, 0.5, 0.5] },
    Vertex { pos: [0.5, -0.5], color: [0.5, 0.5, 0.5] },
    Vertex { pos: [-0.5, -0.5], color: [0.5, 0.5, 0.5] },
];

pub const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
