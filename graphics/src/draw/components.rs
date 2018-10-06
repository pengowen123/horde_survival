//! Components related to graphics

use common::cgmath::{self, Angle};
use specs;

use draw::passes::shadow::LightSpaceMatrix;

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

/// A directional light
///
/// In order to work, an entity must have the `Direction` component in addition to this one
#[derive(Clone, Copy, Debug)]
pub struct DirectionalLight {
    pub color: LightColor,
    pub shadows: Option<LightSpaceMatrix>,
}

impl DirectionalLight {
    /// Creates a new `DirectionalLight` with the provided properties
    pub fn new(color: LightColor, shadows: Option<LightSpaceMatrix>) -> Self {
        Self { color, shadows }
    }
}

/// A point light
///
/// In order to work, an entity must have the `Position` component in addition to this one.
#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub color: LightColor,
    pub attenuation: LightAttenuation,
}

impl PointLight {
    /// Creates a new `PointLight` with the provided properties
    pub fn new(color: LightColor, attenuation: LightAttenuation) -> Self {
        Self { color, attenuation }
    }
}

/// A spot light
///
/// In order to work, an entity must have the `Direction` and `Position` components in addition to
/// this one.
// NOTE: `cos_cutoff` and `cos_outer_cutoff` must be the cosine of the desired angle, in radians.
//       This is enforced by the constructor
#[derive(Clone, Copy, Debug)]
pub struct SpotLight {
    pub color: LightColor,
    cos_cutoff: cgmath::Rad<f32>,
    cos_outer_cutoff: cgmath::Rad<f32>,
    pub attenuation: LightAttenuation,
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
        cutoff: cgmath::Rad<f32>,
        outer_cutoff: cgmath::Rad<f32>,
        attenuation: LightAttenuation,
    ) -> Result<Self, LightError> {
        if cutoff > outer_cutoff {
            Err(LightError::SpotLightAngle(cutoff, outer_cutoff))
        } else {
            Ok(Self {
                color,
                cos_cutoff: cgmath::Rad(cutoff.cos()),
                cos_outer_cutoff: cgmath::Rad(outer_cutoff.cos()),
                attenuation,
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
            display("Spot light cutoff angle was larger than the outer cutoff angle: {:?} > {:?}",
                    cutoff,
                    outer_cutoff)
        }
    }
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
