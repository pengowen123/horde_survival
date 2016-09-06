use gfx;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub const SQUARE: [Vertex; 6] = [
    Vertex { pos: [ -0.5, 0.0 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  0.5, 0.0 ], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [  0.0,  0.666 ], color: [0.0, 0.0, 1.0] },
    Vertex { pos: [  -0.5,  0.0 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  0.5,  0.0 ], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [  0.0,  -0.666 ], color: [0.0, 1.0, 1.0] },
];

pub const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];
