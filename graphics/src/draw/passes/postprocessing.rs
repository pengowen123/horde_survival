//! Postprocessing pass

use gfx::{self, texture, state, handle};
use gfx::traits::FactoryExt;
use rendergraph::pass::Pass;
use rendergraph::framebuffer::Framebuffers;
use rendergraph::error::{RunError, BuildError};
use shred::Resources;
use common::config;
use assets;

use std::collections::HashMap;

use draw::{types, utils, passes};
use draw::passes::resource_pass;
use draw::glsl::{Vec2, Vec4};

gfx_defines! {
    vertex Vertex {
        pos: Vec2 = "a_Pos",
        uv: Vec2 = "a_Uv",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        texture: gfx::TextureSampler<Vec4> = "t_Screen",
        screen_color: gfx::RenderTarget<types::ColorFormat> = "Target0",
   }
}

impl Vertex {
    pub fn new(pos: Vec2, uv: Vec2) -> Self {
        Self { pos, uv }
    }
}

pub struct PostPass<R: gfx::Resources> {
    bundle: gfx::Bundle<R, pipe::Data<R>>,
    enabled: bool,
}

impl<R: gfx::Resources> PostPass<R> {
    fn new<F>(
        factory: &mut F,
        assets: &assets::Assets,
        texture: handle::ShaderResourceView<R, [f32; 4]>,
        main_color: handle::RenderTargetView<R, types::ColorFormat>,
        enabled: bool,
    ) -> Result<Self, BuildError<String>>
        where F: gfx::Factory<R>,
    {
        let pso = Self::load_pso(factory, assets, enabled)?;
        // Create a screen quad to render to
        let vertices = utils::create_screen_quad(|pos, uv| Vertex::new(pos, uv));
        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertices, ());

        // Create texture sampler info
        let sampler_info =
            texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Clamp);

        let data = pipe::Data {
            vbuf,
            texture: (texture, factory.create_sampler(sampler_info)),
            screen_color: main_color,
        };

        Ok(PostPass {
            bundle: gfx::Bundle::new(slice, pso, data),
            enabled,
        })
    }
    
    /// Loads the postprocessing PSO
    ///
    /// The shaders will be the postprocessing shaders if `enabled` is `true`, or a simple
    /// pass-through otherwise.
    fn load_pso<F: gfx::Factory<R>>(factory: &mut F, assets: &assets::Assets, enabled: bool)
        -> Result<gfx::PipelineState<R, pipe::Meta>, BuildError<String>>
    {
        let mut defines = HashMap::new();

        if enabled {
            defines.insert("POSTPROCESSING_ENABLED".into(), "1".into());
        }

        passes::load_pso(
            assets,
            factory,
            "post_vertex.glsl",
            "post_fragment.glsl",
            gfx::Primitive::TriangleList,
            state::Rasterizer::new_fill(),
            pipe::new(),
            defines,
        )
    }
}

pub fn setup_pass<R, C, F>(builder: &mut types::GraphBuilder<R, C, F>)
    -> Result<(), BuildError<String>>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    let main_color = {
        builder.main_color().clone()
    };

    let srv =
        builder
            .get_pass_output::<resource_pass::IntermediateTarget<R>>("intermediate_target")?
            .srv
            .clone();

    let enabled = builder.get_resources().fetch::<config::GraphicsConfig>().postprocessing;

    let pass = PostPass::new(builder.factory, builder.assets, srv, main_color, enabled)?;

    builder.add_pass(pass);

    Ok(())
}

impl<R, C, F> Pass<R, C, F, types::ColorFormat, types::DepthFormat> for PostPass<R>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    fn name(&self) -> &str {
        "postprocessing"
    }

    fn execute_pass(&mut self, encoder: &mut gfx::Encoder<R, C>, _: &mut Resources)
        -> Result<(), RunError>
    {
        self.bundle.encode(encoder);
        
        Ok(())
    }

    fn reload_shaders(
        &mut self,
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<(), BuildError<String>> {
        self.bundle.pso = Self::load_pso(factory, assets, self.enabled)?;
        Ok(())
    }

    fn handle_window_resize(
        &mut self,
        _: (u16, u16),
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        _: &mut F,
    ) -> Result<(), BuildError<String>> {
        let intermediate_target = framebuffers
            .get_framebuffer::<resource_pass::IntermediateTarget<R>>(
                "intermediate_target"
            )?;

        // Update shader input to the resized intermediate target
        self.bundle.data.texture.0 = intermediate_target.srv.clone();

        // Update shader output to the resized main color target
        self.bundle.data.screen_color = framebuffers.get_main_color().clone();

        Ok(())
    }

    fn apply_config(
        &mut self,
        config: &config::GraphicsConfig,
        _: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<(), BuildError<String>> {
        self.bundle.pso = Self::load_pso(factory, assets, config.postprocessing)?;
        self.enabled = config.postprocessing;
        Ok(())
    }
}
