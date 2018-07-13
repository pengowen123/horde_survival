//! Declaration of the geometry pass pipeline
//!
//! This pass calculates position, normal, color, and specular data for each fragment.

use gfx::{self, state, texture, handle};
use gfx::traits::FactoryExt;
use shred;
use rendergraph::pass::Pass;
use rendergraph::error::{RunError, BuildError};
use window::info::WindowInfo;
use cgmath::{Matrix4, SquareMatrix};
use specs::Join;

use std::sync::{Arc, Mutex};

use draw::{DrawableStorageRef, passes, types};
use draw::passes::resource_pass;
use draw::glsl::{Vec2, Vec3, Vec4, Mat4};
use super::gbuffer;
use camera::Camera;
use assets;

pub struct Output<R: gfx::Resources> {
    pub gbuffer: gbuffer::GeometryBuffer<R>,
}

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

pub struct GeometryPass<R: gfx::Resources> {
    // TODO: Skip the Bundle here, just use PSO + Data fields
    bundle: gfx::Bundle<R, pipe::Data<R>>,
}

impl<R: gfx::Resources> GeometryPass<R> {
    pub fn new<F>(
        factory: &mut F,
        (window_width, window_height): (u16, u16),
        dsv: handle::DepthStencilView<R, types::DepthFormat>,
    ) -> Result<(Self, Output<R>), BuildError<String>>
    where
        F: gfx::Factory<R>,
    {
        let pso = Self::load_pso(factory)?;

        // Create dummy data
        let vbuf = factory.create_vertex_buffer(&[]);

        let texels = [[0x0; 4]];
        let (_, texture_view) = factory
            .create_texture_immutable::<gfx::format::Rgba8>(
                texture::Kind::D2(1, 1, texture::AaMode::Single),
                texture::Mipmap::Allocated,
                &[&texels],
            )?;

        // Create texture sampler info
        let sampler_info = texture::SamplerInfo::new(
            texture::FilterMethod::Bilinear,
            texture::WrapMode::Tile,
        );

        // Create geometry buffer
        let gbuffer = gbuffer::GeometryBuffer::new(factory, window_width, window_height)?;

        let data = pipe::Data {
            vbuf,
            locals: factory.create_constant_buffer(1),
            diffuse: (texture_view.clone(), factory.create_sampler(sampler_info)),
            specular: (texture_view, factory.create_sampler(sampler_info)),
            out_pos: gbuffer.position.rtv().clone(),
            out_normal: gbuffer.normal.rtv().clone(),
            out_color: gbuffer.color.rtv().clone(),
            out_depth: dsv.clone(),
        };

        let slice = gfx::Slice::new_match_vertex_buffer(&data.vbuf);

        let pass = GeometryPass {
            bundle: gfx::Bundle::new(slice, pso, data)
        };

        let output = Output { gbuffer };

        Ok((pass, output))
    }
    
    fn load_pso<F: gfx::Factory<R>>(factory: &mut F)
        -> Result<gfx::PipelineState<R, pipe::Meta>, BuildError<String>>
    {
        let rasterizer = state::Rasterizer {
            cull_face: state::CullFace::Back,
            ..state::Rasterizer::new_fill()
        };

        passes::load_pso(
            factory,
            assets::get_shader_path("geometry_pass_vertex"),
            assets::get_shader_path("geometry_pass_fragment"),
            gfx::Primitive::TriangleList,
            rasterizer,
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
    let window_dim = builder.get_resources().fetch::<WindowInfo>(0).physical_dimensions();
    let window_dim: (u32, u32) = window_dim.into();
    
    let dsv = builder
        .get_pass_output::<resource_pass::IntermediateTarget<R>>("intermediate_target")?
        .dsv
        .clone();

    let (pass, output) = {
        GeometryPass::new(builder.factory(), (window_dim.0 as u16, window_dim.1 as u16), dsv)?
    };

    builder.add_pass(pass);
    builder.add_pass_output("gbuffer", output);

    Ok(())
}

impl<R, C, F> Pass<R, C, F> for GeometryPass<R>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    fn execute_pass(&mut self, encoder: &mut gfx::Encoder<R, C>, resources: &mut shred::Resources)
        -> Result<(), RunError>
    {
        encoder.clear(&self.bundle.data.out_pos, [0.0; 4]);
        encoder.clear(&self.bundle.data.out_normal, [0.0; 4]);
        encoder.clear(&self.bundle.data.out_color, [0.0; 4]);

        let camera = resources.fetch::<Arc<Mutex<Camera>>>(0);
        let camera = camera.lock().unwrap();
        let view_proj = camera.projection() * camera.view();

        let drawable = resources.fetch::<DrawableStorageRef<R>>(0);
        let drawable = unsafe { &*drawable.get() };

        let mut locals = Locals {
            model: Matrix4::identity().into(),
            view_proj: view_proj.into(),
        };

        for d in drawable.join() {
            // Get model-specific transform matrix
            let model = d.param().get_model_matrix();

            // Update shader parameters
            locals.model = model.into();
            locals.view_proj = view_proj.into();

            // Update model-specific buffers
            encoder.update_constant_buffer(&self.bundle.data.locals, &locals);

            // TODO: use the entity's material
            //encoder.update_constant_buffer(
                //&data.material,
                //&drawable.material(),
            //);

            // Update texture maps
            self.bundle.data.diffuse.0 = d.diffuse().clone();
            self.bundle.data.specular.0 = d.specular().clone();

            // Update the vertex buffer
            self.bundle.data.vbuf = d.vertex_buffer().clone();

            // Draw the model
            encoder.draw(
                d.slice(),
                &self.bundle.pso,
                &self.bundle.data,
            );
        }
        
        Ok(())
    }

    fn reload_shaders(&mut self, factory: &mut F) -> Result<(), BuildError<String>> {
        self.bundle.pso = Self::load_pso(factory)?;
        Ok(())
    }
}
