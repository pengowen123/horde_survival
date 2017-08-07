//! Components related to drawing entities

use specs;
use gfx;

use super::param;
use super::pipeline::main::{geometry_pass, lighting};
use super::types::{TextureView, VertexBuffer};

/// A component that stores the information needed to draw an entity
pub struct Drawable<R: gfx::Resources> {
    vertex_buffer: VertexBuffer<R, geometry_pass::Vertex>,
    diffuse: TextureView<R>,
    specular: TextureView<R>,
    material: lighting::Material,
    slice: gfx::Slice<R>,
    param: param::ShaderParam,
}

impl<R: gfx::Resources> Drawable<R> {
    /// Returns a new `Drawable`, with the provided texture, vertex buffer, and slice
    pub fn new(
        vertex_buffer: VertexBuffer<R, geometry_pass::Vertex>,
        slice: gfx::Slice<R>,
        diffuse: TextureView<R>,
        specular: TextureView<R>,
        material: lighting::Material,
    ) -> Self {
        Self {
            vertex_buffer,
            slice,
            diffuse,
            specular,
            material,
            param: Default::default(),
        }
    }

    /// Returns a reference to the component's diffuse map
    pub fn diffuse(&self) -> &TextureView<R> {
        &self.diffuse
    }

    /// Returns a reference to the component's specular map
    pub fn specular(&self) -> &TextureView<R> {
        &self.specular
    }

    /// Returns a reference to the component's material
    pub fn material(&self) -> &lighting::Material {
        &self.material
    }

    /// Returns a reference to the component's vertex buffer
    pub fn vertex_buffer(&self) -> &VertexBuffer<R, geometry_pass::Vertex> {
        &self.vertex_buffer
    }

    /// Returns a reference to the component's vertex buffer slice
    pub fn slice(&self) -> &gfx::Slice<R> {
        &self.slice
    }

    /// Returns a reference to the component's shader parameters
    pub fn param(&self) -> &param::ShaderParam {
        &self.param
    }

    /// Sets the shader parameters to the provided value
    pub fn set_shader_param(&mut self, param: param::ShaderParam) {
        self.param = param;
    }
}

pub struct Scale(pub f32);

impl<R: gfx::Resources> specs::Component for Drawable<R> {
    type Storage = specs::VecStorage<Self>;
}

impl specs::Component for Scale {
    type Storage = specs::VecStorage<Self>;
}
