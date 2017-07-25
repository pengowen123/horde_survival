//! Declaration of the graphics pipeline, and a component for containing the information needed to
//! draw an entity

use specs;
use gfx;

use super::param;

/// The color format for graphics
pub type ColorFormat = gfx::format::Srgba8;

/// The depth format for graphics
pub type DepthFormat = gfx::format::DepthStencil;

/// A view into a texture
pub type TextureView<R> = gfx::handle::ShaderResourceView<R, [f32; 4]>;
/// A vertex buffer
pub type VertexBuffer<R> = gfx::handle::Buffer<R, Vertex>;

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

/// A component that stores the information needed to draw an entity
pub struct Drawable<R: gfx::Resources> {
    texture: TextureView<R>,
    vertex_buffer: VertexBuffer<R>,
    slice: gfx::Slice<R>,
    param: param::ShaderParam,
}

impl<R: gfx::Resources> Drawable<R> {
    /// Returns a new `Drawable`, with the provided texture, vertex buffer, and slice
    pub fn new(
        texture: TextureView<R>,
        vertex_buffer: VertexBuffer<R>,
        slice: gfx::Slice<R>,
    ) -> Self {
        Self {
            texture,
            vertex_buffer,
            slice,
            param: Default::default(),
        }
    }

    pub fn texture(&self) -> &TextureView<R> {
        &self.texture
    }

    pub fn vertex_buffer(&self) -> &VertexBuffer<R> {
        &self.vertex_buffer
    }

    pub fn slice(&self) -> &gfx::Slice<R> {
        &self.slice
    }

    pub fn param(&self) -> &param::ShaderParam {
        &self.param
    }

    /// Sets the shader parameters to the provided value
    pub fn set_shader_param(&mut self, param: param::ShaderParam) {
        self.param = param;
    }
}

impl<R: gfx::Resources> specs::Component for Drawable<R> {
    type Storage = specs::VecStorage<Self>;
}
