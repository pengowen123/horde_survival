//! Pipeline declaration for shadows from point lights

use gfx::{self, state, handle, texture};
use gfx::traits::FactoryExt;

use std::path::Path;

use draw::{types, passes};
use draw::factory_ext::FactoryExtension;
use draw::glsl::{Mat4, Vec3};
use draw::passes::main::geometry_pass;

// The number of faces on a cubemap
const CUBE_FACES: usize = 6;

gfx_defines! {
    constant ShadowMatrix {
        matrix: Mat4 = "matrix",
    }
    
    constant Locals {
        model: Mat4 = "model",
        light_pos: Vec3 = "lightPos",
        far_plane: f32 = "farPlane",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<geometry_pass::Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        view_matrices: gfx::ConstantBuffer<ShadowMatrix> = "u_ShadowMatrices",
        out_depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

impl<T: Into<Mat4>> From<T> for ShadowMatrix {
    fn from(matrix: T) -> Self {
        Self { matrix: matrix.into() }
    }
}

// TODO: Handle aspect ratio within the pass execution
/*impl<R: gfx::Resources> Pipeline<R> {*/
    //pub fn new_point_shadow<F, P>(
        //factory: &mut F,
        //shadow_map_size: texture::Size,
        //vs_path: P,
        //gs_path: P,
        //fs_path: P,
    //) -> Result<(Self, handle::ShaderResourceView<R, [f32; 4]>), passes::PassError>
    //where
        //F: gfx::Factory<R>,
        //P: AsRef<Path>,
    //{
        //let pso = passes::load_pso_geometry(
            //factory,
            //vs_path,
            //gs_path,
            //fs_path,
            //gfx::Primitive::TriangleList,
            //state::Rasterizer::new_fill(),
            //pipe::new(),
        //)?;

        //// Create dummy vertex data
        //let vbuf = factory.create_vertex_buffer(&[]);

        //// Create a shadow map
        //let (srv, dsv) = factory.create_depth_stencil_cubemap::<types::DepthFormat>(
            //shadow_map_size,
        //)?;

        //let data = pipe::Data {
            //vbuf,
            //locals: factory.create_constant_buffer(1),
            //view_matrices: factory.create_constant_buffer(CUBE_FACES),
            //out_depth: dsv,
        //};

        //Ok((Pipeline::new(pso, data), srv))
    //}
/*}*/
