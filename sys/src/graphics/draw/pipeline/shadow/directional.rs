//! Pipeline declaration for shadows from directional lights

use gfx::{self, state, handle, texture};
use gfx::traits::FactoryExt;

use std::path::Path;

use graphics::draw::{types, pipeline};
use graphics::draw::pipeline::main::geometry_pass;
use graphics::draw::glsl::Mat4;

gfx_defines! {
    pipeline pipe {
        vbuf: gfx::VertexBuffer<geometry_pass::Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        out_depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }

    constant Locals {
        light_space_matrix: Mat4 = "lightSpaceMatrix",
        model: Mat4 = "model",
    }
}

/// A `Pipeline` for the skybox shaders
pub type Pipeline<R> = pipeline::Pipeline<R, pipe::Data<R>>;

impl<R: gfx::Resources> Pipeline<R> {
    /// Returns a new directional light shadow `Pipeline`, created from the provided shaders, and a
    /// texture view for the shadow map
    pub fn new_dir_shadow<F, P>(
        factory: &mut F,
        shadow_map_size: texture::Size,
        vs_path: P,
        fs_path: P,
    ) -> Result<(Self, handle::ShaderResourceView<R, [f32; 4]>), pipeline::PipelineError>
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

        // Create dummy vertex data
        let vbuf = factory.create_vertex_buffer(&[]);

        // Create a shadow map
        let (_, srv, dsv) = factory.create_depth_stencil(
            shadow_map_size,
            shadow_map_size,
        )?;

        let data = pipe::Data {
            vbuf,
            locals: factory.create_constant_buffer(1),
            out_depth: dsv,
        };

        Ok((Pipeline::new(pso, data), srv))
    }
}
