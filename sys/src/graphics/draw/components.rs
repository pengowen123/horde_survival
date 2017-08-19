//! Components related to graphics

use specs;
use gfx;
use cgmath::{self, Angle};

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

/// The color of a light
#[derive(Clone, Copy, Debug)]
pub struct LightColor {
    pub ambient: [f32; 4],
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
}

impl LightColor {
    pub fn new(ambient: [f32; 4], diffuse: [f32; 4], specular: [f32; 4]) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
        }
    }
}

/// A directional light
///
/// In order to work, an entity must have the `Direction` component in addition to this one.
#[derive(Clone, Copy, Debug)]
pub struct DirectionalLight {
    pub color: LightColor,
}

impl DirectionalLight {
    pub fn new(color: LightColor) -> Self {
        Self { color }
    }
}

/// A point light
///
/// In order to work, an entity must have the `Spatial` component in addition to this one.
#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub color: LightColor,
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl PointLight {
    /// Creates a new `PointLight` with the provided properties
    ///
    /// `constant`, `linear` and `quadratic` are the attenuation properties of the light.
    pub fn new(color: LightColor, constant: f32, linear: f32, quadratic: f32) -> Self {
        Self {
            color,
            constant,
            linear,
            quadratic,
        }
    }
}

/// A spot light
///
/// In order to work, an entity must have the `Direction` and `Spatial` components in addition to
/// this one.
// NOTE: `cos_cutoff` and `cos_outer_cutoff` must be the cosine of the desired angle, in radians.
//       This is be enforced by the constructor
#[derive(Clone, Copy, Debug)]
pub struct SpotLight {
    pub color: LightColor,
    pub cos_cutoff: f32,
    pub cos_outer_cutoff: f32,
}

impl SpotLight {
    /// Creates a new `SpotLight` with the provided properties
    ///
    /// `cutoff` is the angle the spotlight will cover. The light will fade between this angle and
    /// the `outer_cutoff` angle.
    ///
    /// # Panics
    ///
    /// Panics if `outer_cutoff` is a smaller angle than `cutoff`
    pub fn new(
        color: LightColor,
        cutoff: cgmath::Rad<f32>,
        outer_cutoff: cgmath::Rad<f32>,
    ) -> Self {
        assert!(
            cutoff < outer_cutoff,
            "`cutoff` ({}) must be smaller than `outer_cutoff` ({})",
            cutoff.0,
            outer_cutoff.0
        );

        Self {
            color,
            cos_cutoff: cutoff.cos(),
            cos_outer_cutoff: outer_cutoff.cos(),
        }
    }
}

/// A scale to apply to an entity when it is drawn
pub struct Scale(pub f32);

impl<R: gfx::Resources> specs::Component for Drawable<R> {
    type Storage = specs::VecStorage<Self>;
}

impl specs::Component for Scale {
    type Storage = specs::VecStorage<Self>;
}

impl specs::Component for DirectionalLight {
    type Storage = specs::VecStorage<Self>;
}

impl specs::Component for PointLight {
    type Storage = specs::VecStorage<Self>;
}

impl specs::Component for SpotLight {
    type Storage = specs::VecStorage<Self>;
}
