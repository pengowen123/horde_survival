//! Pipeline declaration for the skybox

use gfx::{self, state, handle, format};
use image_utils;

use super::*;
use graphics::draw::{types, utils};
use assets::read_bytes;

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
    /// Returns a new skybox `Pipeline`, created from the provided shaders and pipeline
    /// initialization data
    pub fn new_skybox<F, P>(
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
        let pso = load_pso(
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
        let path = |p| env!("CARGO_MANIFEST_DIR").to_string() + p;
        let cubemap = image_utils::load_cubemap(
            factory,
            image_utils::CubemapData {
                up: &read_bytes(path("/assets/skybox/top.jpg"))?,
                down: &read_bytes(path("/assets/skybox/bottom.jpg"))?,
                front: &read_bytes(path("/assets/skybox/front.jpg"))?,
                back: &read_bytes(path("/assets/skybox/back.jpg"))?,
                left: &read_bytes(path("/assets/skybox/left.jpg"))?,
                right: &read_bytes(path("/assets/skybox/right.jpg"))?,
            },
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
