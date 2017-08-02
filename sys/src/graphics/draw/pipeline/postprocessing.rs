//! Pipeline declaration for postprocessing

use gfx::{self, texture, state, handle};

use super::*;
use graphics::draw::{types, utils};

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

/// A `Pipeline` for the postprocessing shaders
pub type Pipeline<R> = super::Pipeline<R, pipe::Data<R>>;

impl<R: gfx::Resources> Pipeline<R> {
    /// Returns a new postprocessing `Pipeline`, created from the provided shaders and pipeline
    /// initialization data
    pub fn new_post<F, P>(
        factory: &mut F,
        srv: types::TextureView<R>,
        rtv: handle::RenderTargetView<R, types::ColorFormat>,
        vs_path: P,
        fs_path: P,
    ) -> Result<Self, PsoError>
    where
        F: gfx::Factory<R>,
        P: AsRef<Path>,
    {
        let pso = load_pso(
            factory,
            vs_path,
            fs_path,
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
            texture: (srv, factory.create_sampler(sampler_info)),
            screen_color: rtv,
        };

        Ok(Pipeline::new(pso, data))
    }
}
