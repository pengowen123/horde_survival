//! Pipeline declaration for the main shaders

pub mod gbuffer;

use gfx::{self, texture, state, handle, format};

use std::path::Path;

use super::*;
use graphics::draw::types;

gfx_defines! {
    vertex Vertex {
        pos: Vec3 = "a_Pos",
        normal: Vec3 = "a_Normal",
        uv: Vec2 = "a_Uv",
    }

    constant Material {
        shininess: f32 = "u_Material_shininess",
    }

    constant Light {
        position: Vec4 = "u_Light_position",

        ambient: Vec4 = "u_Light_ambient",
        diffuse: Vec4 = "u_Light_diffuse",
        specular: Vec4 = "u_Light_specular",

        // Attenuation properties
        constant: f32 = "u_Light_constant",
        linear: f32 = "u_Light_linear",
        quadratic: f32 = "u_Light_quadratic",
    }

    constant Locals {
        // Transformation matrices
        mvp: Mat4 = "u_MVP",
        model: Mat4 = "u_Model",
        // Position of the camera
        eye_pos: Vec4 = "u_EyePos",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        material: gfx::ConstantBuffer<Material> = "u_Material",
        light: gfx::ConstantBuffer<Light> = "u_Light",
        texture: gfx::TextureSampler<Vec4> = "t_Color",
        texture_diffuse: gfx::TextureSampler<Vec4> = "t_Diffuse",
        texture_specular: gfx::TextureSampler<Vec4> = "t_Specular",
        out_color: gfx::RenderTarget<format::Rgba8> = "Target0",
        out_depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

// Constructors

impl Material {
    pub fn new(shininess: f32) -> Self {
        Self { shininess }
    }
}

impl Light {
    /// Returns a new `Light` with the provided properties
    ///
    /// If the `w` component of the position vector is `0.0`, the light will be directional.
    ///
    /// The `constant`, `linear` and `quadratic` values are used for attenuation calculations.
    pub fn new(
        position: Vec4,
        ambient: Vec4,
        diffuse: Vec4,
        specular: Vec4,
        constant: f32,
        linear: f32,
        quadratic: f32,
    ) -> Self {
        Self {
            position,
            ambient,
            diffuse,
            specular,
            constant,
            linear,
            quadratic,
        }
    }
}

impl Vertex {
    pub fn new(pos: Vec3, uv: Vec2, normal: Vec3) -> Self {
        Self { pos, normal, uv }
    }
}

/// A `Pipeline` for the main shaders
pub type Pipeline<R> = super::Pipeline<R, pipe::Data<R>>;

impl<R: gfx::Resources> Pipeline<R> {
    /// Returns a new main `Pipeline`, created from the provided shaders and pipeline initialization
    /// data
    pub fn new_main<F, P>(
        factory: &mut F,
        rtv: handle::RenderTargetView<R, format::Rgba8>,
        dsv: handle::DepthStencilView<R, types::DepthFormat>,
        vs_path: P,
        fs_path: P,
    ) -> Result<Self, PsoError>
    where
        F: gfx::Factory<R>,
        P: AsRef<Path>,
    {
        // TODO: maybe enable culling
        let rasterizer = state::Rasterizer {
            //samples: Some(state::MultiSample),
            ..state::Rasterizer::new_fill()
        };

        let pso = load_pso(
            factory,
            vs_path,
            fs_path,
            gfx::Primitive::TriangleList,
            rasterizer,
            pipe::new(),
        )?;

        // Create dummy data
        let vbuf = factory.create_vertex_buffer(&[]);

        let texels = [[0x0; 4]];
        let (_, texture_view) = factory
            .create_texture_immutable::<gfx::format::Rgba8>(
                texture::Kind::D2(1, 1, texture::AaMode::Single),
                &[&texels],
            )
            .unwrap();

        // Create texture sampler info
        let sampler_info =
            texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Clamp);

        let data = pipe::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            material: factory.create_constant_buffer(1),
            light: factory.create_constant_buffer(1),
            texture: (texture_view.clone(), factory.create_sampler(sampler_info)),
            texture_diffuse: (texture_view.clone(), factory.create_sampler(sampler_info)),
            texture_specular: (texture_view, factory.create_sampler(sampler_info)),
            out_color: rtv,
            out_depth: dsv,
        };

        Ok(Pipeline::new(pso, data))
    }
}
