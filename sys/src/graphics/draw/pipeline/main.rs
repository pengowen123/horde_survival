use gfx;

use super::*;

gfx_defines! {
    vertex Vertex {
        pos: Vec3 = "a_Pos",
        normal: Vec3 = "a_Normal",
        uv: Vec2 = "a_Uv",
    }

    constant Material {
        shininess: f32 = "u_Material_shininess",
    }

    constant Light {
        position: Vec4 = "u_Light_position",

        ambient: Vec4 = "u_Light_ambient",
        diffuse: Vec4 = "u_Light_diffuse",
        specular: Vec4 = "u_Light_specular",

        // Attenuation properties
        constant: f32 = "u_Light_constant",
        linear: f32 = "u_Light_linear",
        quadratic: f32 = "u_Light_quadratic",
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
        texture_diffuse: gfx::TextureSampler<Vec4> = "t_Diffuse",
        texture_specular: gfx::TextureSampler<Vec4> = "t_Specular",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

// Constructors

impl Material {
    pub fn new(shininess: f32) -> Self {
        Self { shininess }
    }
}

impl Light {
    /// Returns a new `Light` with the provided properties
    ///
    /// If the `w` component of the position vector is `0.0`, the light will be directional.
    ///
    /// The `constant`, `linear` and `quadratic` values are used for attenuation calculations.
    pub fn new(
        position: Vec4,
        ambient: Vec4,
        diffuse: Vec4,
        specular: Vec4,
        constant: f32,
        linear: f32,
        quadratic: f32,
    ) -> Self {
        Self {
            position,
            ambient,
            diffuse,
            specular,
            constant,
            linear,
            quadratic,
        }
    }
}

impl Vertex {
    pub fn new(pos: Vec3, uv: Vec2, normal: Vec3) -> Self {
        Self { pos, normal, uv }
    }
}
