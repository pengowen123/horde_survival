//! Declaration of the lighting pass pipeline
//!
//! Uses the data in the geometry buffer to calculate lighting.

use gfx::{self, format, handle, state, texture};
use gfx::traits::FactoryExt;

use std::path::Path;

use draw::{pipeline, types, components, utils};
use draw::glsl::{Vec2, Vec3, Vec4, Mat4, vec4};
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
    }

    #[derive(Default)]
    constant SpotLight {
        position: Vec4 = "position",
        direction: Vec4 = "direction",

        ambient: Vec4 = "ambient",
        diffuse: Vec4 = "diffuse",
        specular: Vec4 = "specular",

        constant: f32 = "constant",
        linear: f32 = "linear",
        quadratic: f32 = "quadratic",

        cos_cutoff: f32 = "cutOff",
        cos_outer_cutoff: f32 = "outerCutOff",
    }

    constant DirectionalLocals {
        eye_pos: Vec4 = "u_EyePos",
        light_space_matrix: Mat4 = "u_LightSpaceMatrix",
    }

    constant PointLocals {
        eye_pos: Vec4 = "u_EyePos",
        far_plane: f32 = "u_FarPlane",
    }

    constant SpotLocals {
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

impl DirectionalLight {
    pub fn from_components(light: components::DirectionalLight, direction: Vec3) -> Self {
        Self {
            direction: vec4(direction, 0.0),
            ambient: light.color.ambient,
            diffuse: light.color.diffuse,
            specular: light.color.specular,
        }
    }
}

impl PointLight {
    pub fn from_components(light: components::PointLight, position: Vec3) -> Self {
        Self {
            position: vec4(position, 1.0),
            ambient: light.color.ambient,
            diffuse: light.color.diffuse,
            specular: light.color.specular,
            constant: light.attenuation.constant,
            linear: light.attenuation.linear,
            quadratic: light.attenuation.quadratic,
        }
    }
}

impl SpotLight {
    pub fn from_components(light: components::SpotLight, position: Vec3, direction: Vec3) -> Self {
        Self {
            position: vec4(position, 1.0),
            direction: vec4(direction, 0.0),
            ambient: light.color.ambient,
            diffuse: light.color.diffuse,
            specular: light.color.specular,
            constant: light.attenuation.constant,
            linear: light.attenuation.linear,
            quadratic: light.attenuation.quadratic,
            cos_cutoff: light.cos_cutoff().0,
            cos_outer_cutoff: light.cos_outer_cutoff().0,
        }
    }
}

/// Expands to a pipeline declaration for the provided light type, a type alias for the pipeline,
/// and a constructor for the pipeline.
macro_rules! create_light_pipeline {
    ($name:ident,
     $alias_name:ident,
     $constructor_name:ident,
     $light_type:path,
     $locals_type:path,
     ($filtering:ident, $wrap_mode:ident),
    ) => {
        gfx_defines! {
            pipeline $name {
                vbuf: gfx::VertexBuffer<Vertex> = (),
                locals: gfx::ConstantBuffer<$locals_type> = "u_Locals",
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
                out_color: gfx::RenderTarget<format::Rgba8> = "Target0",
                // NOTE: This is `LESS_EQUAL_TEST` instead of `LESS_EQUAL_WRITE`
                out_depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_TEST,
            }
        }

        pub type $alias_name<R> = pipeline::Pipeline<R, $name::Data<R>>;

        impl<R: gfx::Resources> $alias_name<R> {
            /// Returns a new lighting `Pipeline`, created from the provided shaders
            ///
            /// The pipeline will use `rtv` as its render target, and `dsv` as its depth target.
            pub fn $constructor_name<F, P>(
                factory: &mut F,
                shadow_map: handle::ShaderResourceView<R, [f32; 4]>,
                srv_pos: handle::ShaderResourceView<R, gbuffer::GFormat>,
                srv_normal: handle::ShaderResourceView<R, gbuffer::GFormat>,
                srv_color: handle::ShaderResourceView<R, gbuffer::GFormat>,
                srv_previous: handle::ShaderResourceView<R, [f32; 4]>,
                dsv: handle::DepthStencilView<R, types::DepthFormat>,
                rtv: handle::RenderTargetView<R, format::Rgba8>,
                vs_path: P,
                fs_path: P,
            ) -> Result<Self, pipeline::PipelineError>
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
                let sampler_info = texture::SamplerInfo::new(texture::FilterMethod::$filtering,
                                                             texture::WrapMode::Tile);

                let shadow_sampler_info =  texture::SamplerInfo {
                    border: texture::PackedColor::from([1.0; 4]),
                    ..texture::SamplerInfo::new(texture::FilterMethod::Scale, texture::WrapMode::$wrap_mode)
                };

                let data = $name::Data {
                    vbuf: vbuf,
                    material: factory.create_constant_buffer(1),
                    locals: factory.create_constant_buffer(1),
                    light: factory.create_constant_buffer(1),
                    shadow_map: (shadow_map, factory.create_sampler(shadow_sampler_info)),
                    g_position: (srv_pos, factory.create_sampler(sampler_info)),
                    g_normal: (srv_normal, factory.create_sampler(sampler_info)),
                    g_color: (srv_color, factory.create_sampler(sampler_info)),
                    target_color: (srv_previous, factory.create_sampler(sampler_info)),
                    out_color: rtv,
                    out_depth: dsv,
                };

                Ok(pipeline::Pipeline::new(pso, data))
            }
        }
    }
}

// These macro calls will create a new pipeline for each type of light
//
// Each pipeline takes in a single light, calculates lighting and shadows for it, and adds the
// result to the result of the previous iteration

create_light_pipeline!(
    pipe_dir_light,
    PipelineDirLight,
    new_dir_light,
    DirectionalLight,
    DirectionalLocals,
    (Bilinear, Border),
);

create_light_pipeline!(
    pipe_point_light,
    PipelinePointLight,
    new_point_light,
    PointLight,
    PointLocals,
    (Trilinear, Clamp),
);

create_light_pipeline!(
    pipe_spot_light,
    PipelineSpotLight,
    new_spot_light,
    SpotLight,
    SpotLocals,
    (Trilinear, Clamp),
);
