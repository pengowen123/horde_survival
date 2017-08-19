//! Declaration of the lighting pass pipeline
//!
//! Uses the data in the geometry buffer to calculate lighting.

use gfx::{self, format, handle, state, texture};

use std::path::Path;

use graphics::draw::{pipeline, utils};
use graphics::draw::pipeline::*;
use super::gbuffer;

gfx_defines! {
    vertex Vertex {
        pos: Vec2 = "a_Pos",
        uv: Vec2 = "a_Uv",
    }

    constant Material {
        shininess: f32 = "u_Material_shininess",
    }

    #[derive(Default)]
    constant DirectionalLight {
        direction: Vec4 = "direction",

        ambient: Vec4 = "ambient",
        diffuse: Vec4 = "diffuse",
        specular: Vec4 = "specular",

        enabled: i32 = "enabled",

        _padding0: Vec3 = "_padding0",
        _padding: Vec3 = "_padding",
        _padding1: f32 = "_padding1",
    }

    #[derive(Default)]
    constant PointLight {
        position: Vec4 = "position",

        ambient: Vec4 = "ambient",
        diffuse: Vec4 = "diffuse",
        specular: Vec4 = "specular",

        constant: f32 = "constant",
        linear: f32 = "linear",
        quadratic: f32 = "quadratic",

        enabled: i32 = "enabled",
    }

    #[derive(Default)]
    constant SpotLight {
        position: Vec4 = "position",
        direction: Vec4 = "direction",

        ambient: Vec4 = "ambient",
        diffuse: Vec4 = "diffuse",
        specular: Vec4 = "specular",

        cos_cutoff: f32 = "cutOff",
        cos_outer_cutoff: f32 = "outerCutOff",

        enabled: i32 = "enabled",

        _padding: f32 = "_padding",
    }

    constant Locals {
        eye_pos: Vec4 = "u_EyePos",
    }
}

impl Vertex {
    pub fn new(pos: Vec2, uv: Vec2) -> Self {
        Self { pos, uv }
    }
}

impl Material {
    pub fn new(shininess: f32) -> Self {
        Self { shininess }
    }
}

// These macros will create new pipelines for each type of light
// This is necessary because `gfx_defines` doesn't support generics
//
// Each pipeline takes in a single light, calculates lighting and shadows for it, and adds the
// result to the result of the previous iteration

create_light_pipeline!(
    pipe_dir_light,
    PipelineDirLight,
    new_dir_light,
    DirectionalLight
);

create_light_pipeline!(
    pipe_point_light,
    PipelinePointLight,
    new_point_light,
    PointLight
);

create_light_pipeline!(
    pipe_spot_light,
    PipelineSpotLight,
    new_spot_light,
    SpotLight
);
