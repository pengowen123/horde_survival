//! Components related to drawing entities

use specs;
use gfx;

use super::{param, pipeline};

/// A view into a texture
pub type TextureView<R> = gfx::handle::ShaderResourceView<R, [f32; 4]>;
/// A vertex buffer
pub type VertexBuffer<R> = gfx::handle::Buffer<R, pipeline::Vertex>;

/// A component that stores the information needed to draw an entity
pub struct Drawable<R: gfx::Resources> {
    texture: TextureView<R>,
    diffuse: TextureView<R>,
    specular: TextureView<R>,
    vertex_buffer: VertexBuffer<R>,
    slice: gfx::Slice<R>,
    param: param::ShaderParam,
}

impl<R: gfx::Resources> Drawable<R> {
    /// Returns a new `Drawable`, with the provided texture, vertex buffer, and slice
    pub fn new(
        vertex_buffer: VertexBuffer<R>,
        slice: gfx::Slice<R>,
        texture: TextureView<R>,
        diffuse: TextureView<R>,
        specular: TextureView<R>,
    ) -> Self {
        Self {
            vertex_buffer,
            slice,
            texture,
            diffuse,
            specular,
            param: Default::default(),
        }
    }

    /// Returns a reference to the component's texture
    pub fn texture(&self) -> &TextureView<R> {
        &self.texture
    }

    /// Returns a reference to the component's diffuse map
    pub fn diffuse(&self) -> &TextureView<R> {
        &self.diffuse
    }

    /// Returns a reference to the component's specular map
    pub fn specular(&self) -> &TextureView<R> {
        &self.specular
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
