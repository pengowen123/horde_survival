//! Skybox pass

use gfx::{self, state, handle, format};
use gfx::memory::Typed;
use gfx::traits::FactoryExt;
use image_utils;
use assets::{self, read_bytes};
use rendergraph::pass::Pass;
use rendergraph::framebuffer::Framebuffers;
use rendergraph::error::{RunError, BuildError};
use shred::Resources;
use common::config;

use std::sync::{Arc, Mutex};

use draw::{types, passes};
use draw::passes::resource_pass;
use draw::glsl::{Vec2, Vec4, Mat4};
use camera::Camera;

gfx_defines! {
    vertex Vertex {
        pos: Vec2 = "a_Pos",
    }

    constant Locals {
        proj: Mat4 = "u_Proj",
        view: Mat4 = "u_View",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        skybox: gfx::TextureSampler<Vec4> = "t_Skybox",
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        out_color: gfx::RawRenderTarget =
            ("Target0", types::RGBA8, state::ColorMask::all(), None),
        // NOTE: This is `LESS_EQUAL_TEST` instead of `LESS_EQUAL_WRITE`
        depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_TEST,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 2]) -> Self {
        Self { pos }
    }
}

enum SkyboxPso<R: gfx::Resources> {
    Rgba8(gfx::PipelineState<R, pipe::Meta>),
    Srgba8(gfx::PipelineState<R, pipe::Meta>),
}

impl<R: gfx::Resources> SkyboxPso<R> {
    fn get_pso(&self) -> &gfx::PipelineState<R, pipe::Meta> {
        match *self {
            SkyboxPso::Rgba8(ref pso) => pso,
            SkyboxPso::Srgba8(ref pso) => pso,
        }
    }
}

pub struct SkyboxPass<R: gfx::Resources> {
    pso: SkyboxPso<R>,
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,
    postprocessing: bool,
}

impl<R: gfx::Resources> SkyboxPass<R> {
    fn new<F>(
        factory: &mut F,
        rtv: handle::RenderTargetView<R, format::Rgba8>,
        dsv: handle::DepthStencilView<R, types::DepthFormat>,
    ) -> Result<Self, BuildError<String>>
        where F: gfx::Factory<R>,
    {
        let pso = Self::load_pso(factory, types::RGBA8)?;

        // Create a screen quad to render to
        let vertices = [
            Vertex::new([-1.0, -1.0]),
            Vertex::new([1.0, -1.0]),
            Vertex::new([1.0, 1.0]),
            Vertex::new([-1.0, 1.0]),
        ];

        let indices = [0u16, 1, 2, 0, 2, 3];
        
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices, &indices[..]);

        // Create the skybox
        // TODO: load this from individual map files
        let path = |p| format!("{}{}{}", env!("CARGO_MANIFEST_DIR").to_string(), "/../", p);

        let read_image = |s| {
            let p = path(s);
            read_bytes(&p).map_err(|e| BuildError::Io(e, p))
        };
        let cubemap = image_utils::load_cubemap::<_, _, image_utils::Srgba8>(
            factory,
            image_utils::CubemapData {
                up: &read_image("test_assets/skybox/top.jpg")?,
                down: &read_image("test_assets/skybox/bottom.jpg")?,
                front: &read_image("test_assets/skybox/front.jpg")?,
                back: &read_image("test_assets/skybox/back.jpg")?,
                left: &read_image("test_assets/skybox/left.jpg")?,
                right: &read_image("test_assets/skybox/right.jpg")?,
            },
            image_utils::JPEG,
        )?;
        
        let sampler_info = gfx::texture::SamplerInfo::new(
            gfx::texture::FilterMethod::Bilinear,
            gfx::texture::WrapMode::Clamp
        );

        let data = pipe::Data {
            vbuf,
            skybox: (cubemap, factory.create_sampler(sampler_info)),
            locals: factory.create_constant_buffer(1),
            out_color: rtv.raw().clone(),
            depth: dsv,
        };

        Ok(SkyboxPass {
            pso: SkyboxPso::Rgba8(pso),
            data,
            slice,
            postprocessing: true,
        })
    }
    
    fn load_pso<F: gfx::Factory<R>>(factory: &mut F, color_format: format::Format)
        -> Result<gfx::PipelineState<R, pipe::Meta>, BuildError<String>>
    {
        passes::load_pso(
            factory,
            assets::get_shader_path("skybox_vertex"),
            assets::get_shader_path("skybox_fragment"),
            gfx::Primitive::TriangleList,
            state::Rasterizer::new_fill(),
            pipe::Init {
                out_color: ("Target0", color_format, state::ColorMask::all(), None),
                ..pipe::new()
            },
        )
    }
}

pub fn setup_pass<R, C, F>(builder: &mut types::GraphBuilder<R, C, F>)
    -> Result<(), BuildError<String>>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    let (rtv, dsv) = {
        let target =
            builder.get_pass_output::<resource_pass::IntermediateTarget<R>>("intermediate_target")?;
        (target.rtv.clone(), target.dsv.clone())
    };

    let pass = SkyboxPass::new(
        builder.factory(),
        rtv,
        dsv,
    )?;

    builder.add_pass(pass);

    Ok(())
}

impl<R, C, F> Pass<R, C, F, types::ColorFormat, types::DepthFormat> for SkyboxPass<R>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    fn name(&self) -> &str {
        "skybox"
    }

    fn execute_pass(&mut self, encoder: &mut gfx::Encoder<R, C>, resources: &mut Resources)
        -> Result<(), RunError>
    {
        let camera = resources.fetch::<Arc<Mutex<Camera>>>(0);
        let camera = camera.lock().unwrap();
        let locals = Locals {
            proj: camera.projection().into(),
            view: camera.skybox_view().into(),
        };

        encoder.update_constant_buffer(
            &self.data.locals,
            &locals,
        );

        encoder.draw(&self.slice, self.pso.get_pso(), &self.data);
        
        Ok(())
    }

    fn reload_shaders(&mut self, factory: &mut F) -> Result<(), BuildError<String>> {
        match self.pso {
            SkyboxPso::Rgba8(ref mut pso) => {
                *pso = Self::load_pso(factory, types::RGBA8)?;
            }
            SkyboxPso::Srgba8(ref mut pso) => {
                *pso = Self::load_pso(factory, types::SRGBA8)?;
            }
        }

        Ok(())
    }

    fn handle_window_resize(
        &mut self,
        _: (u16, u16),
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        _: &mut F,
    ) -> Result<(), BuildError<String>> {
        let (rtv, dsv) = if self.postprocessing {
            let intermediate_target = framebuffers
                .get_framebuffer::<resource_pass::IntermediateTarget<R>>("intermediate_target")?;

            (intermediate_target.rtv.raw().clone(), intermediate_target.dsv.clone())
        } else {
            (framebuffers.get_main_color().raw().clone(), framebuffers.get_main_depth().clone())
        };

        // Update shader outputs to the resized targets
        self.data.out_color = rtv;
        self.data.depth = dsv;

        Ok(())
    }

    fn apply_config(
        &mut self,
        config: &config::GraphicsConfig,
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        factory: &mut F,
    ) -> Result<(), BuildError<String>> {
        // If the postprocessing setting was disabled, use the main targets
        if !config.postprocessing && self.postprocessing {
            println!("skybox pass: disabling postprocessing");
            self.data.out_color = framebuffers.get_main_color().raw().clone();
            self.data.depth = framebuffers.get_main_depth().clone();

            self.pso = SkyboxPso::Srgba8(Self::load_pso(factory, types::SRGBA8)?);
        }

        // If the postprocessing setting was enabled, use the intermediate targets
        if config.postprocessing && !self.postprocessing {
            println!("skybox pass: enabling postprocessing");
            let intermediate_target = framebuffers
                .get_framebuffer::<resource_pass::IntermediateTarget<R>>("intermediate_target")?;

            self.data.out_color = intermediate_target.rtv.raw().clone();
            self.data.depth = intermediate_target.dsv.clone();

            self.pso = SkyboxPso::Rgba8(Self::load_pso(factory, types::RGBA8)?);
        }

        self.postprocessing = config.postprocessing;

        Ok(())
    }
}
