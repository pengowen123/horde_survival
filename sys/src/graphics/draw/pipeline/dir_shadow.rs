//! Pipeline declaration for shadows from directional lights

use gfx::{self, state, handle, texture};

use super::*;
use graphics::draw::types;
use graphics::draw::pipeline::main::geometry_pass;

gfx_defines! {
    constant Locals {
        light_space_matrix: Mat4 = "lightSpaceMatrix",
        model: Mat4 = "model",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<geometry_pass::Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        out_depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

/// A `Pipeline` for the skybox shaders
pub type Pipeline<R> = super::Pipeline<R, pipe::Data<R>>;

impl<R: gfx::Resources> Pipeline<R> {
    /// Returns a new directional light shadow `Pipeline`, created from the provided shaders, and a
    /// texture view for the depth target
    ///
    /// The pipeline will use `dsv` as its depth target.
    pub fn new_dir_shadow<F, P>(
        factory: &mut F,
        shadow_map_size: texture::Size,
        vs_path: P,
        fs_path: P,
    ) -> Result<(Self, handle::ShaderResourceView<R, [f32; 4]>), PipelineError>
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
