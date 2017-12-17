//! Pipeline declaration for the skybox

use gfx::{self, state, handle, format};
use gfx::traits::FactoryExt;
use image_utils;
use assets::read_bytes;

use std::path::Path;

use draw::{types, utils, pipeline};
use draw::glsl::{Vec3, Vec4, Mat4};

gfx_defines! {
    vertex Vertex {
        pos: Vec3 = "a_Pos",
    }

    constant Locals {
        view_proj: Mat4 = "u_ViewProj",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        skybox: gfx::TextureSampler<Vec4> = "t_Skybox",
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        out_color: gfx::RenderTarget<format::Rgba8> = "Target0",
        out_depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3]) -> Self {
        Self { pos }
    }
}

/// A `Pipeline` for the skybox shaders
pub type Pipeline<R> = super::Pipeline<R, pipe::Data<R>>;

impl<R: gfx::Resources> Pipeline<R> {
    /// Returns a new skybox `Pipeline`, created from the provided shaders
    ///
    /// The pipeline will use `rtv` as its render target, and `dsv` as its depth target.
    pub fn new_skybox<F, P>(
        factory: &mut F,
        rtv: handle::RenderTargetView<R, format::Rgba8>,
        dsv: handle::DepthStencilView<R, types::DepthFormat>,
        vs_path: P,
        fs_path: P,
    ) -> Result<Self, pipeline::PipelineError>
    where
        F: gfx::Factory<R>,
        P: AsRef<Path>,
    {
        let pso = pipeline::load_pso(
            factory,
            vs_path,
            fs_path,
            gfx::Primitive::TriangleList,
            state::Rasterizer::new_fill(),
            pipe::new(),
        )?;

        // Create a screen quad to render to
        let vertices = utils::create_skybox_cube(|pos| Vertex::new(pos));
        let vbuf = factory.create_vertex_buffer(&vertices);

        // Create the skybox
        // TODO: load this from individual map files
        let path = |p| env!("CARGO_MANIFEST_DIR").to_string() + p;

        let cubemap = image_utils::load_cubemap::<_, _, image_utils::Srgba8>(
            factory,
            image_utils::CubemapData {
                up: &read_bytes(path("/test_assets/skybox/top.jpg"))?,
                down: &read_bytes(path("/test_assets/skybox/bottom.jpg"))?,
                front: &read_bytes(path("/test_assets/skybox/front.jpg"))?,
                back: &read_bytes(path("/test_assets/skybox/back.jpg"))?,
                left: &read_bytes(path("/test_assets/skybox/left.jpg"))?,
                right: &read_bytes(path("/test_assets/skybox/right.jpg"))?,
            },
            image_utils::JPEG,
        )?;

        let data = pipe::Data {
            vbuf,
            skybox: (cubemap, factory.create_sampler_linear()),
            locals: factory.create_constant_buffer(1),
            out_color: rtv,
            out_depth: dsv,
        };

        Ok(Pipeline::new(pso, data))
    }
}
