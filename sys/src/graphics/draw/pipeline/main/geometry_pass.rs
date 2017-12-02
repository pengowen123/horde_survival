//! Declaration of the geometry pass pipeline
//!
//! This pass calculates position, normal, color, and specular data for each fragment.

use gfx::{self, handle, state, texture};
use gfx::traits::FactoryExt;

use std::path::Path;

use graphics::draw::{pipeline, types};
use graphics::draw::glsl::{Vec2, Vec3, Vec4, Mat4};
use super::gbuffer;

gfx_defines! {
    vertex Vertex {
        pos: Vec3 = "a_Pos",
        normal: Vec3 = "a_Normal",
        uv: Vec2 = "a_Uv",
    }

    constant Locals {
        model: Mat4 = "u_Model",
        view_proj: Mat4 = "u_ViewProj",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        diffuse: gfx::TextureSampler<Vec4> = "t_Diffuse",
        specular: gfx::TextureSampler<Vec4> = "t_Specular",
        out_pos: gfx::RenderTarget<gbuffer::GFormat> = "Target0",
        out_normal: gfx::RenderTarget<gbuffer::GFormat> = "Target1",
        out_color: gfx::RenderTarget<gbuffer::GFormat> = "Target2",
        out_depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: Vec3, uv: Vec2, normal: Vec3) -> Self {
        Self { pos, normal, uv }
    }
}

pub type Pipeline<R> = pipeline::Pipeline<R, pipe::Data<R>>;

impl<R: gfx::Resources> Pipeline<R> {
    /// Returns a new geometry pass `Pipeline`, created from the provided shaders
    ///
    /// The pipeline will use `rtv` as its render target, and `dsv` as its depth target.
    pub fn new_geometry_pass<F, P>(
        factory: &mut F,
        rtv_pos: handle::RenderTargetView<R, gbuffer::GFormat>,
        rtv_normal: handle::RenderTargetView<R, gbuffer::GFormat>,
        rtv_color: handle::RenderTargetView<R, gbuffer::GFormat>,
        dsv: handle::DepthStencilView<R, types::DepthFormat>,
        vs_path: P,
        fs_path: P,
    ) -> Result<Self, pipeline::PipelineError>
    where
        F: gfx::Factory<R>,
        P: AsRef<Path>,
    {
        // TODO: maybe enable culling
        let rasterizer = state::Rasterizer {
            //samples: Some(state::MultiSample),
            cull_face: state::CullFace::Back,
            ..state::Rasterizer::new_fill()
        };

        let pso = pipeline::load_pso(
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
        let sampler_info = texture::SamplerInfo::new(
            texture::FilterMethod::Anisotropic(8),
            texture::WrapMode::Tile,
        );

        let data = pipe::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            diffuse: (texture_view.clone(), factory.create_sampler(sampler_info)),
            specular: (texture_view, factory.create_sampler(sampler_info)),
            out_pos: rtv_pos,
            out_normal: rtv_normal,
            out_color: rtv_color,
            out_depth: dsv,
        };

        Ok(pipeline::Pipeline::new(pso, data))
    }
}
