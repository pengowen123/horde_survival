//! Postprocessing pass

use gfx::{self, texture, state, handle};
use gfx::traits::FactoryExt;
use rendergraph::pass::Pass;
use shred::Resources;
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
}

impl<R: gfx::Resources> PostPass<R> {
    fn new<F>(
        factory: &mut F,
        texture: handle::ShaderResourceView<R, [f32; 4]>,
        main_color: handle::RenderTargetView<R, types::ColorFormat>,
    ) -> Result<Self, passes::PassError>
        where F: gfx::Factory<R>,
    {
        let pso = passes::load_pso(
            factory,
            assets::get_shader_path("post_vertex"),
            assets::get_shader_path("post_fragment"),
            gfx::Primitive::TriangleList,
            state::Rasterizer::new_fill(),
            pipe::new(),
        )?;

        // Create a screen quad to render to
        let vertices = utils::create_screen_quad(|pos, uv| Vertex::new(pos, uv));
        let vbuf = factory.create_vertex_buffer(&vertices);

        // Create texture sampler info
        let sampler_info =
            texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Clamp);

        let data = pipe::Data {
            vbuf: vbuf,
            texture: (texture, factory.create_sampler(sampler_info)),
            screen_color: main_color,
        };

        let slice = gfx::Slice::new_match_vertex_buffer(&data.vbuf);

        Ok(PostPass {
            bundle: gfx::Bundle::new(slice, pso, data),
        })
    }
}

pub fn setup_pass<R, C, F>(builder: &mut types::GraphBuilder<R, C, F>)
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    let main_color = {
        builder.main_color().clone()
    };

    let srv = builder.get_pass_output::<resource_pass::IntermediateTarget<R>>("intermediate_target")
                     .unwrap()
                     .srv
                     .clone();

    let pass = PostPass::new(builder.factory(), srv, main_color).unwrap();

    builder.add_pass(pass);
}

impl<R, C> Pass<R, C> for PostPass<R>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
{
    fn execute_pass(&mut self, encoder: &mut gfx::Encoder<R, C>, _: &mut Resources) {
        self.bundle.encode(encoder);
    }
}
