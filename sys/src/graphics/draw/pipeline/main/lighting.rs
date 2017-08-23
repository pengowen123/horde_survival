//! Declaration of the lighting pass pipeline
//!
//! Uses the data in the geometry buffer to calculate lighting.

use gfx::{self, format, handle, state, texture};
use cgmath;

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
        light_space_matrix: Mat4 = "u_LightSpaceMatrix",
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

/// A trait implemented by all light types, used for getting the transform matrix for calculating
/// a shadow map from the light's perspective
pub trait LightShadowInfo {
    fn get_light_space_transform(&self) -> cgmath::Matrix4<f32>;
}

impl LightShadowInfo for DirectionalLight {
    fn get_light_space_transform(&self) -> cgmath::Matrix4<f32> {
        // TODO: find a better way to get position of directional light
        let light_pos = cgmath::Point3::new(-15.0, 15.0, 7.5);
        let direction: cgmath::Vector3<f32> =
            [self.direction[0], self.direction[1], self.direction[2]].into();

        let view =
            cgmath::Matrix4::look_at(light_pos, light_pos + direction, cgmath::Vector3::unit_z());

        // TODO: decide on the best values here
        let proj = cgmath::ortho(-20.0, 20.0, -20.0, 20.0, 1.0, 70.0);

        (proj * view)
    }
}

/// Expands to a pipeline declaration for the provided light type, a type alias for the pipeline,
/// and a constructor for the pipeline.
macro_rules! create_light_pipeline {
    ($name:ident, $alias_name:ident, $constructor_name:ident, $light_type:path, ($filtering:ident, $wrap_mode:ident),) => {
        gfx_defines! {
            pipeline $name {
                vbuf: gfx::VertexBuffer<Vertex> = (),
                locals: gfx::ConstantBuffer<Locals> = "u_Locals",
                material: gfx::ConstantBuffer<Material> = "u_Material",
                // The light to process
                light: gfx::ConstantBuffer<$light_type> = "u_Light",
                // Shadow map
                shadow_map: gfx::TextureSampler<Vec4> = "t_ShadowMap",
                // G-buffer textures
                g_position: gfx::TextureSampler<Vec4> = "t_Position",
                g_normal: gfx::TextureSampler<Vec4> = "t_Normal",
                g_color: gfx::TextureSampler<Vec4> = "t_Color",
                target_color: gfx::TextureSampler<Vec4> = "t_Target",
                // Output color (note that depth is not needed here)
                out_color: gfx::RenderTarget<format::Rgba8> = "Target0",
            }
        }

        pub type $alias_name<R> = pipeline::Pipeline<R, $name::Data<R>>;

        impl<R: gfx::Resources> $alias_name<R> {
            /// Returns a new lighting `Pipeline`, created from the provided shaders
            ///
            /// The pipeline will use `rtv` as its render target.
            pub fn $constructor_name<F, P>(
                factory: &mut F,
                shadow_map: handle::ShaderResourceView<R, [f32; 4]>,
                srv_pos: handle::ShaderResourceView<R, gbuffer::GFormat>,
                srv_normal: handle::ShaderResourceView<R, gbuffer::GFormat>,
                srv_color: handle::ShaderResourceView<R, gbuffer::GFormat>,
                srv_previous: handle::ShaderResourceView<R, [f32; 4]>,
                rtv: handle::RenderTargetView<R, format::Rgba8>,
                vs_path: P,
                fs_path: P,
            ) -> Result<Self, PipelineError>
            where
                F: gfx::Factory<R>,
                P: AsRef<Path>,
            {
                let rasterizer = state::Rasterizer { ..state::Rasterizer::new_fill() };

                let pso = pipeline::load_pso(
                    factory,
                    vs_path,
                    fs_path,
                    gfx::Primitive::TriangleList,
                    rasterizer,
                    $name::new(),
                )?;

                // Create a screen quad
                let vertices = utils::create_screen_quad(|pos, uv| Vertex::new(pos, uv));
                let vbuf = factory.create_vertex_buffer(&vertices);

                // Create texture sampler info
                let sampler_info =
                    texture::SamplerInfo::new(texture::FilterMethod::$filtering, texture::WrapMode::$wrap_mode);

                let data = $name::Data {
                    vbuf: vbuf,
                    material: factory.create_constant_buffer(1),
                    locals: factory.create_constant_buffer(1),
                    light: factory.create_constant_buffer(1),
                    shadow_map: (shadow_map, factory.create_sampler(sampler_info)),
                    g_position: (srv_pos, factory.create_sampler(sampler_info)),
                    g_normal: (srv_normal, factory.create_sampler(sampler_info)),
                    g_color: (srv_color, factory.create_sampler(sampler_info)),
                    target_color: (srv_previous, factory.create_sampler(sampler_info)),
                    out_color: rtv,
                };

                Ok(Pipeline::new(pso, data))
            }
        }
    }
}

// These macro calls will create a new pipeline for each type of light
//
// Each pipeline takes in a single light, calculates lighting and shadows for it, and adds the
// result to the result of the previous iteration

// TODO: Think about sampler info creation settings

create_light_pipeline!(
    pipe_dir_light,
    PipelineDirLight,
    new_dir_light,
    DirectionalLight,
    (Bilinear, Clamp),
);

create_light_pipeline!(
    pipe_point_light,
    PipelinePointLight,
    new_point_light,
    PointLight,
    (Trilinear, Clamp),
);

create_light_pipeline!(
    pipe_spot_light,
    PipelineSpotLight,
    new_spot_light,
    SpotLight,
    (Trilinear, Clamp),
);
