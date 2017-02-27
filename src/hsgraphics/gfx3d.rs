use gfx;

pub use hsgraphics::ColorFormat;

pub type DepthFormat = gfx::format::DepthStencil;

/// Types used for gfx
/// 3d vertices contain a position and a texture coordinate
gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        uv: [f32; 2] = "a_Uv",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: ::gfx::VertexBuffer<Vertex> = (),
        transform: ::gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        locals: ::gfx::ConstantBuffer<Locals> = "Locals",
        color: ::gfx::TextureSampler<[f32; 4]> = "t_Color",
        out_color: ::gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: ::gfx::DepthTarget<DepthFormat> =
            ::gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], uv: [f32; 2]) -> Vertex {
        Vertex {
            pos: [pos[0], pos[1], pos[2], 1.0],
            uv: uv,
        }
    }
}
