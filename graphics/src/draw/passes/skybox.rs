//! Skybox pass

use gfx::{self, state, handle, format};
use gfx::traits::FactoryExt;
use image_utils;
use rendergraph::pass::Pass;
use rendergraph::framebuffer::Framebuffers;
use rendergraph::error::{RunError, BuildError};
use shred::Resources;
use common::config;
use assets::{self, read_bytes};

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

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
        out_color: gfx::RenderTarget<format::Rgba8> = "Target0",
        // NOTE: This is `LESS_EQUAL_TEST` instead of `LESS_EQUAL_WRITE`
        depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_TEST,
    }
}

impl Vertex {
    pub fn new(pos: [f32; 2]) -> Self {
        Self { pos }
    }
}

pub struct SkyboxPass<R: gfx::Resources> {
    bundle: gfx::Bundle<R, pipe::Data<R>>,
}

impl<R: gfx::Resources> SkyboxPass<R> {
    fn new<F>(
        factory: &mut F,
        assets: &assets::Assets,
        rtv: handle::RenderTargetView<R, format::Rgba8>,
        dsv: handle::DepthStencilView<R, types::DepthFormat>,
    ) -> Result<Self, BuildError<String>>
        where F: gfx::Factory<R>,
    {
        let pso = Self::load_pso(factory, assets)?;

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
        let skybox_dir = assets.get_assets_dir().join("skybox");
        let path = |p| skybox_dir.join(p);

        let read_image = |s| {
            let p = path(s);
            read_bytes(&p).map_err(|e| {
                let p = p
                    .to_str()
                    .expect("Skybox image path contained invalid unicode")
                    .to_string();
                BuildError::Io(e, p)
            })
        };
        let cubemap = image_utils::load_cubemap::<_, _, image_utils::Srgba8>(
            factory,
            image_utils::CubemapData {
                up: &read_image("top.jpg")?,
                down: &read_image("bottom.jpg")?,
                front: &read_image("front.jpg")?,
                back: &read_image("back.jpg")?,
                left: &read_image("left.jpg")?,
                right: &read_image("right.jpg")?,
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
            out_color: rtv.clone(),
            depth: dsv,
        };

        Ok(SkyboxPass {
            bundle: gfx::Bundle::new(slice, pso, data),
        })
    }
    
    fn load_pso<F: gfx::Factory<R>>(factory: &mut F, assets: &assets::Assets)
        -> Result<gfx::PipelineState<R, pipe::Meta>, BuildError<String>>
    {
        passes::load_pso(
            assets,
            factory,
            "skybox_vertex.glsl",
            "skybox_fragment.glsl",
            gfx::Primitive::TriangleList,
            state::Rasterizer::new_fill(),
            pipe::new(),
            HashMap::new(),
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
        builder.factory,
        builder.assets,
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
        let camera = resources.fetch::<Arc<Mutex<Camera>>>();
        let camera = camera.lock().unwrap();
        let locals = Locals {
            proj: camera.projection().into(),
            view: camera.skybox_view().into(),
        };

        encoder.update_constant_buffer(
            &self.bundle.data.locals,
            &locals,
        );

        self.bundle.encode(encoder);
        
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

            (intermediate_target.rtv.clone(), intermediate_target.dsv.clone())
        };

        // Update shader outputs to the resized targets
        self.bundle.data.out_color = rtv;
        self.bundle.data.depth = dsv;

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
