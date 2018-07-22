//! Postprocessing pass

use gfx::{self, texture, state, handle, format};
use gfx::traits::FactoryExt;
use rendergraph::pass::Pass;
use rendergraph::framebuffer::Framebuffers;
use rendergraph::error::{RunError, BuildError};
use shred::Resources;
use common::config;
use assets;

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
        texture: handle::ShaderResourceView<R, [f32; 4]>,
        main_color: handle::RenderTargetView<R, types::ColorFormat>,
    ) -> Result<Self, BuildError<String>>
        where F: gfx::Factory<R>,
    {
        let pso = Self::load_pso(factory)?;
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
            enabled: true,
        })
    }
    
    fn load_pso<F: gfx::Factory<R>>(factory: &mut F)
        -> Result<gfx::PipelineState<R, pipe::Meta>, BuildError<String>>
    {
        passes::load_pso(
            factory,
            assets::get_shader_path("post_vertex"),
            assets::get_shader_path("post_fragment"),
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
    let main_color = {
        builder.main_color().clone()
    };

    let srv =
        builder
            .get_pass_output::<resource_pass::IntermediateTarget<R>>("intermediate_target")?
            .srv
            .clone();

    let pass = PostPass::new(builder.factory(), srv, main_color)?;

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
        if !self.enabled {
            return Ok(());
        }

        self.bundle.encode(encoder);
        
        Ok(())
    }

    fn reload_shaders(&mut self, factory: &mut F) -> Result<(), BuildError<String>> {
        self.bundle.pso = Self::load_pso(factory)?;
        Ok(())
    }

    fn handle_window_resize(
        &mut self,
        _: (u16, u16),
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        _: &mut F,
    ) -> Result<(), BuildError<String>> {
        if !self.enabled {
            return Ok(());
        }

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
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        factory: &mut F,
    ) -> Result<(), BuildError<String>> {
        // If the postprocessing setting was disabled, use a dummy texture as the shader input (the
        // intermediate targets don't exist if postprocessing is disabled)
        if !config.postprocessing && self.enabled {
            println!("postprocessing pass: disabling postprocessing");
            let texels = [[0x0; 4]];
            let (_, srv) = factory
                .create_texture_immutable::<format::Rgba8>(
                    texture::Kind::D2(1, 1, texture::AaMode::Single),
                    texture::Mipmap::Provided,
                    &[&texels],
                )?;

            self.bundle.data.texture.0 = srv.clone();
        }

        // If the postprocessing setting was enabled, use the intermediate target as the shader
        // input
        if config.postprocessing && !self.enabled {
            println!("postprocessing pass: disabling postprocessing");
            let intermediate_target = framebuffers
                .get_framebuffer::<resource_pass::IntermediateTarget<R>>("intermediate_target")?;

            self.bundle.data.texture.0 = intermediate_target.srv.clone();
        }

        self.enabled = config.postprocessing;

        Ok(())
    }
}
