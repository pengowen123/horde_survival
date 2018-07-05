//! Pipeline declaration for shadows from directional lights

use gfx::{self, state, handle, texture};
use gfx::traits::FactoryExt;
use specs::Join;
use cgmath::{Matrix4, SquareMatrix};
use rendergraph::pass::Pass;
use rendergraph::error::{RunError, BuildError};
use shred;

use std::sync::{Arc, Mutex};

use draw::{DrawableStorageRef, types, passes};
use draw::passes::main::geometry_pass;
use draw::passes::shadow;
use draw::glsl::Mat4;
use assets;

pub struct Output<R: gfx::Resources> {
    pub srv: handle::ShaderResourceView<R, [f32; 4]>,
}

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

pub struct DirectionalShadowPass<R: gfx::Resources> {
    bundle: gfx::Bundle<R, pipe::Data<R>>,
}

impl<R: gfx::Resources> DirectionalShadowPass<R> {
    fn new<F: gfx::Factory<R>>(
        factory: &mut F,
        shadow_map_size: texture::Size,
    ) -> Result<(Self, Output<R>), BuildError<String>> {
        let (_, srv, dsv) = factory.create_depth_stencil(
            shadow_map_size,
            shadow_map_size,
        )?;

        let vbuf = factory.create_vertex_buffer(&[]);
        let slice = gfx::Slice::new_match_vertex_buffer(&vbuf);
        
        let data = pipe::Data {
            vbuf,
            locals: factory.create_constant_buffer(1),
            out_depth: dsv,
        };

        let pso = Self::load_pso(factory)?;
        let pass = Self {
            bundle: gfx::Bundle::new(slice, pso, data),
        };

        let output = Output {
            srv,
        };
        
        Ok((pass, output))
    }

    fn load_pso<F: gfx::Factory<R>>(factory: &mut F)
        -> Result<gfx::PipelineState<R, pipe::Meta>, BuildError<String>>
    {
        passes::load_pso(
            factory,
            assets::get_shader_path("dir_shadow_vertex"),
            assets::get_shader_path("dir_shadow_fragment"),
            gfx::Primitive::TriangleList,
            state::Rasterizer::new_fill(),
            pipe::new(),
        )
    }
}

pub fn setup_pass<R, C, F>(builder: &mut types::GraphBuilder<R, C, F>)
    -> Result<(), BuildError<String>>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    let (pass, output) = DirectionalShadowPass::new({builder.factory()}, super::SHADOW_MAP_SIZE)?;

    builder.add_pass(pass);
    builder.add_pass_output("dir_shadow_map", output);

    Ok(())
}


impl<R, C, F> Pass<R, C, F> for DirectionalShadowPass<R>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    fn execute_pass(&mut self, encoder: &mut gfx::Encoder<R, C>, resources: &mut shred::Resources)
        -> Result<(), RunError>
    {
        encoder.clear_depth(&self.bundle.data.out_depth, 1.0);

        let drawable = resources.fetch::<DrawableStorageRef<R>>(0);
        let drawable = unsafe { &*drawable.get() };
        
        let shadow_source = resources.fetch::<Arc<Mutex<shadow::DirShadowSource>>>(0).clone();
        let light_space_matrix = match shadow_source.lock().unwrap().light_space_matrix() {
            Some(m) => m,
            // If there is no shadow source, just return (depth buffer was already cleared)
            None => return Ok(()),
        };
        let mut locals = Locals {
            light_space_matrix: light_space_matrix.into(),
            model: Matrix4::identity().into(),
        };

        for d in drawable.join() {
            let model = d.param().get_model_matrix();
            locals.model = model.into();
            
            encoder.update_constant_buffer(&self.bundle.data.locals, &locals);
            
            self.bundle.data.vbuf = d.vertex_buffer().clone();
            self.bundle.slice = d.slice().clone();
            self.bundle.encode(encoder);
        }
        
        Ok(())
    }

    fn reload_shaders(&mut self, factory: &mut F) -> Result<(), BuildError<String>> {
        self.bundle.pso = Self::load_pso(factory)?;
        Ok(())
    }
}
