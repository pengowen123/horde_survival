//! Graphics pipeline declaration

use gfx;

/// The color format for graphics
pub type ColorFormat = gfx::format::Srgba8;

/// The depth format for graphics
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        normal: [f32; 3] = "a_Normal",
        uv: [f32; 2] = "a_Uv",
    }

    constant Locals {
        // Transformation matrices
        mvp:        [[f32; 4]; 4] = "u_MVP",
        model_view: [[f32; 4]; 4] = "u_ModelView",
        model:      [[f32; 4]; 4] = "u_Model",

        // Lighting
        light_pos: [f32; 4] = "u_LightPos",
        light_color: [f32; 4] = "u_LightColor",
        ambient_color: [f32; 4] = "u_AmbientColor",
        eye_pos: [f32; 4] = "u_EyePos",
        light_strength: f32 = "u_LightStrength",
        ambient_strength: f32 = "u_AmbientStrength",
        specular_strength: f32 = "u_SpecularStrength",
        specular_focus: f32 = "u_SpecularFocus",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        texture: gfx::TextureSampler<[f32; 4]> = "t_Color",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], uv: [f32; 2], normal: [f32; 3]) -> Self {
        Self { pos, normal, uv }
    }
}
