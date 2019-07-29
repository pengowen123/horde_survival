//! Declaration of the geometry pass pipeline
//!
//! This pass calculates position, normal, color, and specular data for each fragment.

use assets;
use cgmath::{Matrix4, SquareMatrix};
use common::config;
use common::graphics::{Vertex, VertexSkeletal};
use common::skeletal_animation::dual_quaternion::DualQuaternion;
use gfx::traits::FactoryExt;
use gfx::memory::Typed;
use gfx::{self, handle, state, texture};
use rendergraph::error::{BuildError, RunError};
use rendergraph::framebuffer::Framebuffers;
use rendergraph::pass::Pass;
use rendergraph::resources::TemporaryResources;
use shred;
use specs::Join;
use window::info::WindowInfo;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::gbuffer;
use camera::Camera;
use draw::glsl::{Mat4, Vec4};
use draw::passes::resource_pass;
use draw::{passes, types};

pub const MAX_JOINTS: usize = 32;

pub struct Output<R: gfx::Resources> {
    pub gbuffer: gbuffer::GeometryBuffer<R>,
}

gfx_defines! {
    constant Locals {
        model: Mat4 = "u_Model",
        view_proj: Mat4 = "u_ViewProj",
    }

    constant LocalsSkeletal {
        model: Mat4 = "u_Model",
        view_proj: Mat4 = "u_ViewProj",
        model_view: Mat4 = "u_ModelView",
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

    pipeline pipe_skeletal {
        vbuf: gfx::VertexBuffer<VertexSkeletal> = (),
        locals: gfx::ConstantBuffer<LocalsSkeletal> = "u_Locals",
        joint_transforms: gfx::RawConstantBuffer = "u_JointTransforms",
        diffuse: gfx::TextureSampler<Vec4> = "t_Diffuse",
        specular: gfx::TextureSampler<Vec4> = "t_Specular",
        out_pos: gfx::RenderTarget<gbuffer::GFormat> = "Target0",
        out_normal: gfx::RenderTarget<gbuffer::GFormat> = "Target1",
        out_color: gfx::RenderTarget<gbuffer::GFormat> = "Target2",
        out_depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

pub struct GeometryPass<R: gfx::Resources> {
    // TODO: Skip the Bundle here, just use PSO + Data fields
    bundle: gfx::Bundle<R, pipe::Data<R>>,
    bundle_skeletal: gfx::Bundle<R, pipe_skeletal::Data<R>>,
    joint_transforms: handle::Buffer<R, DualQuaternion<f32>>,
}

impl<R: gfx::Resources> GeometryPass<R> {
    pub fn new<F>(
        factory: &mut F,
        assets: &assets::Assets,
        (window_width, window_height): (u16, u16),
        dsv: handle::DepthStencilView<R, types::DepthFormat>,
    ) -> Result<(Self, Output<R>), BuildError<String>>
    where
        F: gfx::Factory<R>,
    {
        let pso = Self::load_pso(factory, assets)?;
        let pso_skeletal = Self::load_pso_skeletal(factory, assets)?;

        // Create dummy data
        let vbuf = factory.create_vertex_buffer(&[]);
        let vbuf_skeletal = factory.create_vertex_buffer(&[]);

        let texels = [[0x0; 4]];
        let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
            texture::Kind::D2(1, 1, texture::AaMode::Single),
            texture::Mipmap::Allocated,
            &[&texels],
        )?;

        // Create texture sampler info
        let sampler_info =
            texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Tile);

        // Create geometry buffer
        let gbuffer = gbuffer::GeometryBuffer::new(factory, window_width, window_height)?;

        let data = pipe::Data {
            vbuf,
            locals: factory.create_constant_buffer(1),
            diffuse: (texture_view.clone(), factory.create_sampler(sampler_info)),
            specular: (texture_view.clone(), factory.create_sampler(sampler_info)),
            out_pos: gbuffer.position.rtv().clone(),
            out_normal: gbuffer.normal.rtv().clone(),
            out_color: gbuffer.color.rtv().clone(),
            out_depth: dsv.clone(),
        };

        let slice = gfx::Slice::new_match_vertex_buffer(&data.vbuf);
        let joint_transforms: handle::Buffer<R, DualQuaternion<f32>> =
            factory.create_constant_buffer(MAX_JOINTS);

        let data_skeletal = pipe_skeletal::Data {
            vbuf: vbuf_skeletal,
            locals: factory.create_constant_buffer(1),
            joint_transforms: joint_transforms.raw().clone(),
            diffuse: (texture_view.clone(), factory.create_sampler(sampler_info)),
            specular: (texture_view, factory.create_sampler(sampler_info)),
            out_pos: gbuffer.position.rtv().clone(),
            out_normal: gbuffer.normal.rtv().clone(),
            out_color: gbuffer.color.rtv().clone(),
            out_depth: dsv.clone(),
        };

        let slice_skeletal = gfx::Slice::new_match_vertex_buffer(&data_skeletal.vbuf);

        let pass = GeometryPass {
            bundle: gfx::Bundle::new(slice_skeletal, pso, data),
            bundle_skeletal: gfx::Bundle::new(slice, pso_skeletal, data_skeletal),
            joint_transforms,
        };

        let output = Output { gbuffer };

        Ok((pass, output))
    }

    fn load_pso<F: gfx::Factory<R>>(
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<gfx::PipelineState<R, pipe::Meta>, BuildError<String>> {
        let rasterizer = state::Rasterizer {
            cull_face: state::CullFace::Back,
            ..state::Rasterizer::new_fill()
        };

        passes::load_pso(
            assets,
            factory,
            "geometry_pass_vertex.glsl",
            "geometry_pass_fragment.glsl",
            gfx::Primitive::TriangleList,
            rasterizer,
            pipe::new(),
            HashMap::new(),
        )
    }

    fn load_pso_skeletal<F: gfx::Factory<R>>(
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<gfx::PipelineState<R, pipe_skeletal::Meta>, BuildError<String>> {
        let rasterizer = state::Rasterizer {
            cull_face: state::CullFace::Back,
            ..state::Rasterizer::new_fill()
        };

        let mut defines = HashMap::new();

        defines.insert("MAX_JOINTS".into(), format!("{}", MAX_JOINTS));

        passes::load_pso(
            assets,
            factory,
            "geometry_pass_vertex_skeletal.glsl",
            "geometry_pass_fragment.glsl",
            gfx::Primitive::TriangleList,
            rasterizer,
            pipe_skeletal::new(),
            defines,
        )
    }
}

pub fn setup_pass<R, C, F>(
    builder: &mut types::GraphBuilder<R, C, F>,
) -> Result<(), BuildError<String>>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    F: gfx::Factory<R>,
{
    let window_dim = builder
        .get_resources()
        .fetch::<WindowInfo>()
        .physical_dimensions();
    let window_dim: (u32, u32) = window_dim.into();

    let dsv = builder
        .get_pass_output::<resource_pass::IntermediateTarget<R>>("intermediate_target")?
        .dsv
        .clone();

    let (pass, output) = GeometryPass::new(
        builder.factory,
        builder.assets,
        (window_dim.0 as u16, window_dim.1 as u16),
        dsv,
    )?;

    builder.add_pass(pass);
    builder.add_pass_output("gbuffer", output);

    Ok(())
}

impl<R, C, F> Pass<R, C, F, types::ColorFormat, types::DepthFormat> for GeometryPass<R>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    F: gfx::Factory<R>,
{
    fn name(&self) -> &str {
        "geometry"
    }

    fn execute_pass(
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        resources: &mut shred::Resources,
        temporary_resources: TemporaryResources<R>,
    ) -> Result<(), RunError> {
        encoder.clear(&self.bundle.data.out_pos, [0.0; 4]);
        encoder.clear(&self.bundle.data.out_normal, [0.0; 4]);
        encoder.clear(&self.bundle.data.out_color, [0.0; 4]);

        let camera = resources.fetch::<Arc<Mutex<Camera>>>();
        let camera = camera.lock().unwrap();
        let view_proj = camera.projection() * camera.view();
        let view = camera.view();

        let mut locals = Locals {
            model: Matrix4::identity().into(),
            view_proj: view_proj.into(),
        };

        for d in temporary_resources.drawable.join() {
            // Get model-specific transform matrix
            let model = d.param().get_model_matrix();

            // Update shader parameters
            locals.model = model.into();

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
            encoder.draw(d.slice(), &self.bundle.pso, &self.bundle.data);
        }

        let mut locals_skeletal = LocalsSkeletal {
            model: Matrix4::identity().into(),
            view_proj: view_proj.into(),
            model_view: Matrix4::identity().into(),
        };

        for d in temporary_resources.drawable_skeletal.join() {
            let skinning_transforms = d.skinning_transforms();

            // Get model-specific transform matrix
            let model = d.param().get_model_matrix();

            // Update shader parameters
            locals_skeletal.model = model.into();
            locals_skeletal.model_view = (view * model).into();

            // Update model-specific buffers
            encoder.update_constant_buffer(&self.bundle_skeletal.data.locals, &locals_skeletal);
            encoder.update_buffer(&self.joint_transforms, &skinning_transforms, 0)?;

            // TODO: use the entity's material
            //encoder.update_constant_buffer(
            //&data.material,
            //&drawable.material(),
            //);

            // Update texture maps
            self.bundle_skeletal.data.diffuse.0 = d.diffuse().clone();
            self.bundle_skeletal.data.specular.0 = d.specular().clone();

            // Update the vertex buffer
            self.bundle_skeletal.data.vbuf = d.vertex_buffer().clone();

            // Draw the model
            encoder.draw(d.slice(), &self.bundle_skeletal.pso, &self.bundle_skeletal.data);
        }

        Ok(())
    }

    fn reload_shaders(
        &mut self,
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<(), BuildError<String>> {
        self.bundle.pso = Self::load_pso(factory, assets)?;
        self.bundle_skeletal.pso = Self::load_pso_skeletal(factory, assets)?;
        Ok(())
    }

    fn handle_window_resize(
        &mut self,
        new_dimensions: (u16, u16),
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        factory: &mut F,
    ) -> Result<(), BuildError<String>> {
        // Build a new geometry buffer using the new window dimensions
        let gbuffer = gbuffer::GeometryBuffer::new(factory, new_dimensions.0, new_dimensions.1)?;

        // Update shader outputs to the resized geometry buffer targets
        self.bundle.data.out_pos = gbuffer.position.rtv().clone();
        self.bundle.data.out_normal = gbuffer.normal.rtv().clone();
        self.bundle.data.out_color = gbuffer.color.rtv().clone();

        self.bundle_skeletal.data.out_pos = gbuffer.position.rtv().clone();
        self.bundle_skeletal.data.out_normal = gbuffer.normal.rtv().clone();
        self.bundle_skeletal.data.out_color = gbuffer.color.rtv().clone();
        // Update shader depth outputs to the resized depth target
        self.bundle.data.out_depth = framebuffers
            .get_framebuffer::<resource_pass::IntermediateTarget<R>>("intermediate_target")?
            .dsv
            .clone();

        self.bundle_skeletal.data.out_depth = self.bundle.data.out_depth.clone();

        framebuffers.add_framebuffer("gbuffer", gbuffer);

        Ok(())
    }

    fn apply_config(
        &mut self,
        _: &config::GraphicsConfig,
        _: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        _: &mut F,
        _: &assets::Assets,
    ) -> Result<(), BuildError<String>> {
        Ok(())
    }
}
