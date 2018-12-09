//! Declaration of the particle pass pipeline
//!
//! Renders particles from each particle source.

use assets;
use common::config;
use common::graphics::MAX_PARTICLES;
use common::specs::Join;
use gfx::state::{self, Blend, BlendValue, Equation, Factor};
use gfx::traits::FactoryExt;
use gfx::{self, buffer, format, handle, memory, texture};
use rendergraph::error::{BuildError, RunError};
use rendergraph::framebuffer::Framebuffers;
use rendergraph::pass::Pass;
use rendergraph::resources::TemporaryResources;
use shred::Resources;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use camera::Camera;
use draw::glsl::{Mat4, Vec2, Vec3, Vec4};
use draw::passes::resource_pass;
use draw::{passes, types};

fn get_blend_function() -> Blend {
    Blend::new(
        Equation::Add,
        Factor::ZeroPlus(BlendValue::SourceAlpha),
        Factor::OneMinus(BlendValue::SourceAlpha),
    )
}
gfx_defines! {
    vertex Vertex {
        pos: Vec3 = "a_Pos",
        uv: Vec2 = "a_Uv",
    }

    vertex Instance {
        translate: Vec3 = "a_Translate",
        alpha: f32 = "a_Alpha",
    }

    constant Locals {
        view_proj: Mat4 = "u_ViewProj",
        camera_right_world_space: Vec3 = "u_CameraRightWorldSpace",
        _padding: f32 = "_padding",
        camera_up_world_space: Vec3 = "u_CameraUpWorldSpace",
        scale: f32 = "u_Scale",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        instance: gfx::InstanceBuffer<Instance> = (),
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        texture: gfx::TextureSampler<Vec4> = "u_Texture",
        out_color: gfx::BlendTarget<format::Rgba8> = (
            "Target0",
            state::ColorMask::all(),
            get_blend_function(),
        ),
        depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_TEST,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], uv: [f32; 2]) -> Self {
        Self { pos, uv }
    }
}

pub struct ParticlePass<R: gfx::Resources> {
    bundle: gfx::Bundle<R, pipe::Data<R>>,
    enabled: bool,
}

impl<R: gfx::Resources> ParticlePass<R> {
    fn new<F>(
        factory: &mut F,
        assets: &assets::Assets,
        rtv: handle::RenderTargetView<R, format::Rgba8>,
        dsv: handle::DepthStencilView<R, types::DepthFormat>,
        enabled: bool,
    ) -> Result<Self, BuildError<String>>
    where
        F: gfx::Factory<R>,
    {
        let pso = Self::load_pso(factory, assets)?;

        // Create a screen quad to render to
        let vertices = [
            Vertex::new([-0.5, -0.5, 0.0], [0.0, 1.0]),
            Vertex::new([-0.5, 0.5, 0.0], [0.0, 0.0]),
            Vertex::new([0.5, 0.5, 0.0], [1.0, 0.0]),
            Vertex::new([0.5, -0.5, 0.0], [1.0, 1.0]),
        ];

        let indices = [0u16, 1, 2, 0, 2, 3];

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices, &indices[..]);

        let instance = factory.create_buffer(
            MAX_PARTICLES,
            buffer::Role::Vertex,
            memory::Usage::Dynamic,
            memory::Bind::TRANSFER_DST,
        )?;

        // Create dummy data
        let texels = [[0x0; 4]];
        let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
            texture::Kind::D2(1, 1, texture::AaMode::Single),
            texture::Mipmap::Allocated,
            &[&texels],
        )?;

        // Create texture sampler info
        let sampler_info =
            texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Tile);

        let data = pipe::Data {
            vbuf,
            instance,
            locals: factory.create_constant_buffer(1),
            texture: (texture_view, factory.create_sampler(sampler_info)),
            out_color: rtv.clone(),
            depth: dsv,
        };

        Ok(ParticlePass {
            bundle: gfx::Bundle::new(slice, pso, data),
            enabled,
        })
    }

    fn load_pso<F: gfx::Factory<R>>(
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<gfx::PipelineState<R, pipe::Meta>, BuildError<String>> {
        passes::load_pso(
            assets,
            factory,
            "particle_vertex.glsl",
            "particle_fragment.glsl",
            gfx::Primitive::TriangleList,
            state::Rasterizer::new_fill(),
            pipe::new(),
            HashMap::new(),
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
    let (rtv, dsv) = {
        let target = builder
            .get_pass_output::<resource_pass::IntermediateTarget<R>>("intermediate_target")?;
        (target.rtv.clone(), target.dsv.clone())
    };

    let enabled = builder
        .get_resources()
        .fetch::<config::GraphicsConfig>()
        .particles;

    let pass = ParticlePass::new(builder.factory, builder.assets, rtv, dsv, enabled)?;

    builder.add_pass(pass);

    Ok(())
}

impl<R, C, F> Pass<R, C, F, types::ColorFormat, types::DepthFormat> for ParticlePass<R>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    F: gfx::Factory<R>,
{
    fn name(&self) -> &str {
        "particles"
    }

    fn execute_pass(
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        resources: &mut Resources,
        temporary_resources: TemporaryResources<R>,
    ) -> Result<(), RunError> {
        let camera = resources.fetch::<Arc<Mutex<Camera>>>();
        let camera = camera.lock().unwrap();
        let view = camera.view();
        let locals = Locals {
            view_proj: (camera.projection() * view).into(),
            camera_right_world_space: [view[0][0], view[1][0], view[2][0]],
            _padding: 0.0,
            camera_up_world_space: [view[0][1], view[1][1], view[2][1]],
            scale: 1.0,
        };

        if self.enabled {
            encoder.update_constant_buffer(&self.bundle.data.locals, &locals);

            for (particle_source,) in (temporary_resources.particle_source,).join() {
                let particles = particle_source
                    .particles()
                    .iter()
                    .filter_map(|p| {
                        if p.lifetime() > 0.0 {
                            Some(Instance {
                                translate: p.position().into(),
                                alpha: p.alpha(),
                            })
                        } else {
                            None
                        }
                    }).collect::<Vec<_>>();

                encoder.update_buffer(&self.bundle.data.instance, &particles, 0)?;

                self.bundle.slice.instances = Some((particles.len() as u32, 0));

                self.bundle.data.texture.0 = particle_source.texture().clone();

                self.bundle.encode(encoder);
            }
        }

        Ok(())
    }

    fn reload_shaders(
        &mut self,
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<(), BuildError<String>> {
        self.bundle.pso = Self::load_pso(factory, assets)?;

        Ok(())
    }

    fn handle_window_resize(
        &mut self,
        _: (u16, u16),
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        _: &mut F,
    ) -> Result<(), BuildError<String>> {
        let (rtv, dsv) = {
            let intermediate_target = framebuffers
                .get_framebuffer::<resource_pass::IntermediateTarget<R>>("intermediate_target")?;

            (
                intermediate_target.rtv.clone(),
                intermediate_target.dsv.clone(),
            )
        };

        // Update shader outputs to the resized targets
        self.bundle.data.out_color = rtv;
        self.bundle.data.depth = dsv;

        Ok(())
    }

    fn apply_config(
        &mut self,
        config: &config::GraphicsConfig,
        _: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        _: &mut F,
        _: &assets::Assets,
    ) -> Result<(), BuildError<String>> {
        self.enabled = config.particles;

        Ok(())
    }
}
