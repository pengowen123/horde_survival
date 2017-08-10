//! Declaration of the lighting pass pipeline
//!
//! This pass calculates lighting using the data calculated by the geometry pass.

use gfx::{self, format, handle, state, texture};

use std::path::Path;

use graphics::draw::{pipeline, utils};
use graphics::draw::pipeline::*;
use super::gbuffer;

/// The maximum number of lights
// NOTE: Changes to this constant must be also applied to the shaders
pub const MAX_LIGHTS: usize = 8;

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

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        material: gfx::ConstantBuffer<Material> = "u_Material",
        // Light buffers
        dir_lights: gfx::ConstantBuffer<DirectionalLight> = "u_DirLights",
        point_lights: gfx::ConstantBuffer<PointLight> = "u_PointLights",
        spot_lights: gfx::ConstantBuffer<SpotLight> = "u_SpotLights",
        // G-buffer textures
        g_position: gfx::TextureSampler<Vec4> = "t_Position",
        g_normal: gfx::TextureSampler<Vec4> = "t_Normal",
        g_color: gfx::TextureSampler<Vec4> = "t_Color",
        // Output color (note that depth is not needed here)
        out_color: gfx::RenderTarget<format::Rgba8> = "Target0",
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

pub type Pipeline<R> = pipeline::Pipeline<R, pipe::Data<R>>;

impl<R: gfx::Resources> Pipeline<R> {
    /// Returns a new main `Pipeline`, created from the provided shaders and pipeline initialization
    /// data
    pub fn new_lighting<F, P>(
        factory: &mut F,
        srv_pos: handle::ShaderResourceView<R, gbuffer::GFormat>,
        srv_normal: handle::ShaderResourceView<R, gbuffer::GFormat>,
        srv_color: handle::ShaderResourceView<R, gbuffer::GFormat>,
        rtv: handle::RenderTargetView<R, format::Rgba8>,
        vs_path: P,
        fs_path: P,
    ) -> Result<Self, PsoError>
    where
        F: gfx::Factory<R>,
        P: AsRef<Path>,
    {
        let rasterizer = state::Rasterizer { ..state::Rasterizer::new_fill() };

        let pso = load_pso(
            factory,
            vs_path,
            fs_path,
            gfx::Primitive::TriangleList,
            rasterizer,
            pipe::new(),
        )?;

        // Create a screen quad
        let vertices = utils::create_screen_quad(|pos, uv| Vertex::new(pos, uv));
        let vbuf = factory.create_vertex_buffer(&vertices);

        // Create texture sampler info
        let sampler_info =
            texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Clamp);

        let data = pipe::Data {
            vbuf: vbuf,
            material: factory.create_constant_buffer(1),
            locals: factory.create_constant_buffer(1),
            dir_lights: factory.create_constant_buffer(MAX_LIGHTS),
            point_lights: factory.create_constant_buffer(MAX_LIGHTS),
            spot_lights: factory.create_constant_buffer(MAX_LIGHTS),
            g_position: (srv_pos, factory.create_sampler(sampler_info)),
            g_normal: (srv_normal, factory.create_sampler(sampler_info)),
            g_color: (srv_color, factory.create_sampler(sampler_info)),
            out_color: rtv,
        };

        Ok(Pipeline::new(pso, data))
    }
}
