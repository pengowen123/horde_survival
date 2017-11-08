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

/// Attenuation properties of a light
#[derive(Clone, Copy, Debug)]
pub struct LightAttenuation {
    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl LightAttenuation {
    pub fn new(constant: f32, linear: f32, quadratic: f32) -> Self {
        LightAttenuation {
            constant,
            linear,
            quadratic,
        }
    }

    // TODO: Maybe provide constructor that takes only a light radius as an argument
}

/// Settings for shadows for a light
#[derive(Clone, Copy, Debug)]
pub enum ShadowSettings {
    Enabled,
    Disabled,
}

/// Info for a projection matrix
#[derive(Clone, Copy, Debug)]
pub struct ProjectionData {
    near: f32,
    far: f32,
}

impl ProjectionData {
    pub fn new(near: f32, far: f32) -> Self {
        Self { near, far }
    }

    pub fn near(&self) -> f32 {
        self.near
    }

    pub fn far(&self) -> f32 {
        self.far
    }
}

/// A directional light
///
/// In order to work, an entity must have the `Spatial` and `Direction` components in addition to
/// this one. The `Spatial` component will be used for rendering a shadow map from the perspective
/// of the light if shadows are enabled.
#[derive(Clone, Copy, Debug)]
pub struct DirectionalLight {
    pub color: LightColor,
    pub shadows: ShadowSettings,
    pub projection_matrix: cgmath::Ortho<f32>,
}

impl DirectionalLight {
    /// Creates a new `DirectionalLight` with the provided properties
    pub fn new(
        color: LightColor,
        shadows: ShadowSettings,
        projection_matrix: cgmath::Ortho<f32>,
    ) -> Self {
        Self {
            color,
            shadows,
            projection_matrix,
        }
    }
}

/// A point light
///
/// In order to work, an entity must have the `Spatial` component in addition to this one.
#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub color: LightColor,
    pub shadows: ShadowSettings,
    pub attenuation: LightAttenuation,
    pub projection: ProjectionData,
}

impl PointLight {
    /// Creates a new `PointLight` with the provided properties
    pub fn new(
        color: LightColor,
        shadows: ShadowSettings,
        attenuation: LightAttenuation,
        projection: ProjectionData,
    ) -> Self {
        Self {
            color,
            shadows,
            attenuation,
            projection,
        }
    }
}

/// A spot light
///
/// In order to work, an entity must have the `Direction` and `Spatial` components in addition to
/// this one.
// NOTE: `cos_cutoff` and `cos_outer_cutoff` must be the cosine of the desired angle, in radians.
//       This is enforced by the constructor
#[derive(Clone, Copy, Debug)]
pub struct SpotLight {
    pub color: LightColor,
    pub shadows: ShadowSettings,
    cos_cutoff: cgmath::Rad<f32>,
    cos_outer_cutoff: cgmath::Rad<f32>,
    pub projection: ProjectionData,
}

impl SpotLight {
    /// Creates a new `SpotLight` with the provided properties
    ///
    /// `cutoff` is the angle the spotlight will cover. The light will fade between this angle and
    /// the `outer_cutoff` angle.
    ///
    /// Returns `Err` if `outer_cutoff` is a smaller angle than `cutoff`
    pub fn new(
        color: LightColor,
        shadows: ShadowSettings,
        cutoff: cgmath::Rad<f32>,
        outer_cutoff: cgmath::Rad<f32>,
        projection: ProjectionData,
    ) -> Result<Self, LightError> {
        if cutoff > outer_cutoff {
            Err(LightError::SpotLightAngle(cutoff, outer_cutoff))
        } else {
            Ok(Self {
                color,
                shadows,
                cos_cutoff: cgmath::Rad(cutoff.cos()),
                cos_outer_cutoff: cgmath::Rad(outer_cutoff.cos()),
                projection,
            })
        }
    }

    /// Returns the cosine of the cutoff angle of the spot light
    pub fn cos_cutoff(&self) -> cgmath::Rad<f32> {
        self.cos_cutoff
    }

    /// Returns the cosine of the outer cutoff angle of the spot light
    pub fn cos_outer_cutoff(&self) -> cgmath::Rad<f32> {
        self.cos_outer_cutoff
    }
}

quick_error! {
    /// An error while creating a light
    #[derive(Debug)]
    pub enum LightError {
        SpotLightAngle(cutoff: cgmath::Rad<f32>, outer_cutoff: cgmath::Rad<f32>) {
            display("Spot light cutoff angle was larger than the outer cutoff angle: {:?} > {:?}", cutoff, outer_cutoff)
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
