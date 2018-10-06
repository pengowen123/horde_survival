//! Common components for graphics
//!
//! Mainly here so the `rendergraph` crate can access them.

use cgmath::{self, One};
use gfx::{self, handle};
use specs;

mod particles;

pub use self::particles::*;

/// A view into a texture
pub type TextureView<R> = handle::ShaderResourceView<R, [f32; 4]>;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        normal: [f32; 3] = "a_Normal",
        uv: [f32; 2] = "a_Uv",
    }

    constant Material {
        shininess: f32 = "u_Material_shininess",
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], uv: [f32; 2], normal: [f32; 3]) -> Self {
        Self { pos, normal, uv }
    }
}

impl Material {
    pub fn new(shininess: f32) -> Self {
        Self { shininess }
    }
}

/// A component that stores the information needed to draw an entity
#[derive(Clone)]
pub struct Drawable<R: gfx::Resources> {
    vertex_buffer: handle::Buffer<R, Vertex>,
    diffuse: TextureView<R>,
    specular: TextureView<R>,
    slice: gfx::Slice<R>,
    material: Material,
    param: ShaderParam,
}

impl<R: gfx::Resources> Drawable<R> {
    /// Returns a new `Drawable`, with the provided texture, vertex buffer, and slice
    pub fn new(
        vertex_buffer: handle::Buffer<R, Vertex>,
        slice: gfx::Slice<R>,
        diffuse: TextureView<R>,
        specular: TextureView<R>,
        material: Material,
    ) -> Self {
        Drawable {
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
    pub fn material(&self) -> &Material {
        &self.material
    }

    /// Returns a reference to the component's vertex buffer
    pub fn vertex_buffer(&self) -> &handle::Buffer<R, Vertex> {
        &self.vertex_buffer
    }

    /// Returns a reference to the component's vertex buffer slice
    pub fn slice(&self) -> &gfx::Slice<R> {
        &self.slice
    }

    /// Returns a reference to the component's shader parameters
    pub fn param(&self) -> &ShaderParam {
        &self.param
    }

    /// Sets the shader parameters to the provided value
    pub fn set_shader_param(&mut self, param: ShaderParam) {
        self.param = param;
    }
}

/// A 3D rotation
pub type Rotation = cgmath::Matrix4<f32>;

/// A 3D scale
pub type Scale = cgmath::Matrix4<f32>;

/// A 3D translation
pub type Translation = cgmath::Matrix4<f32>;

/// A type that stores all individual parameters to pass to the graphics system
#[derive(Clone, Copy, Debug)]
pub struct ShaderParam {
    translation: Translation,
    rotation: Rotation,
    scale: Scale,
}

impl ShaderParam {
    pub fn new(translation: Translation, rotation: Rotation, scale: Scale) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn set_translation(&mut self, new_translation: Translation) {
        self.translation = new_translation;
    }

    pub fn set_rotation(&mut self, new_rotation: Rotation) {
        self.rotation = new_rotation;
    }

    pub fn set_scale(&mut self, new_scale: Scale) {
        self.scale = new_scale;
    }

    /// Returns the model matrix, created from the stored translation, rotation, and scale matrices
    pub fn get_model_matrix(&self) -> cgmath::Matrix4<f32> {
        self.translation * self.rotation * self.scale
    }
}

impl Default for ShaderParam {
    fn default() -> Self {
        // Identity transformations (zero translation, zero rotation)
        ShaderParam::new(One::one(), One::one(), One::one())
    }
}

impl specs::Component for ShaderParam {
    type Storage = specs::DenseVecStorage<Self>;
}

impl<R: gfx::Resources> specs::Component for Drawable<R> {
    type Storage = specs::DenseVecStorage<Self>;
}
