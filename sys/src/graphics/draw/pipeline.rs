//! Graphics pipeline declaration

use gfx;

/// The color format for graphics
pub type ColorFormat = gfx::format::Srgba8;

/// The depth format for graphics
pub type DepthFormat = gfx::format::DepthStencil;

/// A GLSL `vec2`
pub type Vec2 = [f32; 2];
/// A GLSL `vec3`
pub type Vec3 = [f32; 3];
/// A GLSL `vec4`
pub type Vec4 = [f32; 4];
/// A GLSL `mat4`
pub type Mat4 = [Vec4; 4];

gfx_defines! {
    vertex Vertex {
        pos: Vec3 = "a_Pos",
        normal: Vec3 = "a_Normal",
        uv: Vec2 = "a_Uv",
    }

    constant Material {
        ambient: Vec3 = "u_Material_ambient",
        padding_0: f32 = "padding_0",
        diffuse: Vec3 = "u_Material_diffuse",
        padding_1: f32 = "padding_1",
        specular: Vec3 = "u_Material_specular",
        padding_2: f32 = "padding_2",
        shininess: f32 = "u_Material_shininess",
    }

    constant Light {
        position: Vec3 = "u_Light_position",
        padding_3: f32 = "padding_3",
        ambient: Vec3 = "u_Light_ambient",
        padding_4: f32 = "padding_4",
        diffuse: Vec3 = "u_Light_diffuse",
        padding_5: f32 = "padding_5",
        specular: Vec3 = "u_Light_specular",
    }

    constant Locals {
        // Transformation matrices
        mvp: Mat4 = "u_MVP",
        model: Mat4 = "u_Model",
        // Position of the camera
        eye_pos: Vec4 = "u_EyePos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        material: gfx::ConstantBuffer<Material> = "u_Material",
        light: gfx::ConstantBuffer<Light> = "u_Light",
        texture: gfx::TextureSampler<Vec4> = "t_Color",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

// Constructors

impl Material {
    pub fn new(ambient: Vec3, diffuse: Vec3, specular: Vec3, shininess: f32) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            shininess,
            padding_0: 0.0,
            padding_1: 0.0,
            padding_2: 0.0,
        }
    }
}

impl Light {
    pub fn new(position: Vec3, ambient: Vec3, diffuse: Vec3, specular: Vec3) -> Self {
        Self {
            position,
            ambient,
            diffuse,
            specular,
            padding_3: 0.0,
            padding_4: 0.0,
            padding_5: 0.0,
        }
    }
}

impl Vertex {
    pub fn new(pos: Vec3, uv: Vec2, normal: Vec3) -> Self {
        Self { pos, normal, uv }
    }
}
