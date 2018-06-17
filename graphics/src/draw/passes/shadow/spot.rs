//! Pipeline declaration for shadows from spot lights

use gfx::{self, state, handle, texture};
use gfx::traits::FactoryExt;

use std::path::Path;

use draw::{types, passes};
use draw::glsl::Mat4;
use draw::passes::main::geometry_pass;

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


// TODO: Handle aspect ratio within the pass execution
/*impl<R: gfx::Resources> Pipeline<R> {*/
    //pub fn new_spot_shadow<F, P>(
        //factory: &mut F,
        //shadow_map_size: texture::Size,
        //vs_path: P,
        //fs_path: P,
    //) -> Result<(Self, handle::ShaderResourceView<R, [f32; 4]>), passes::PassError>
    //where
        //F: gfx::Factory<R>,
        //P: AsRef<Path>,
    //{
        //let pso = passes::load_pso(
            //factory,
            //vs_path,
            //fs_path,
            //gfx::Primitive::TriangleList,
            //state::Rasterizer::new_fill(),
            //pipe::new(),
        //)?;

        //// Create dummy vertex data
        //let vbuf = factory.create_vertex_buffer(&[]);

        //// Create a shadow map
        //let (_, srv, dsv) = factory.create_depth_stencil::<types::DepthFormat>(
            //shadow_map_size,
            //shadow_map_size,
        //)?;

        //let data = pipe::Data {
            //vbuf,
            //locals: factory.create_constant_buffer(1),
            //out_depth: dsv,
        //};

        //Ok((Pipeline::new(pso, data), srv))
    //}
/*}*/
