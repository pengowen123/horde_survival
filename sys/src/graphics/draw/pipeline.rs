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
        ambient: Vec3 = "ambient",
        diffuse: Vec3 = "diffuse",
        specular: Vec3 = "specular",
        shininess: f32 = "shininess",
    }

    constant Light {
        position: Vec3 = "position",
        ambient: Vec3 = "ambient",
        diffuse: Vec3 = "diffuse",
        specular: Vec3 = "specular",
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
        }
    }
}

impl Vertex {
    pub fn new(pos: Vec3, uv: Vec2, normal: Vec3) -> Self {
        Self { pos, normal, uv }
    }
}
