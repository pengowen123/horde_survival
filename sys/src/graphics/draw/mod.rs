//! Rendering system
//!
//! Draws each entity that has a `Drawable` component and handles displaying the results to the
//! window.

mod shader;

pub use self::shader::{Vertex, ColorFormat, DepthFormat, Drawable};
pub use self::shader::pipe::new as init_pipeline;

use specs::{self, Join};
use gfx::{self, texture};
use gfx::traits::FactoryExt;
use graphics::{window, camera};

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

// TODO: Write docs for these type aliases when I figure out what they do
pub type OutColor<R> = gfx::handle::RenderTargetView<R, shader::ColorFormat>;
pub type OutDepth<R> = gfx::handle::DepthStencilView<R, shader::DepthFormat>;

pub struct System<F, C, R, D>
    where F: gfx::Factory<R>,
          C: gfx::CommandBuffer<R>,
          R: gfx::Resources,
          D: gfx::Device<Resources = R, CommandBuffer = C>
{
    factory: F,
    encoder: gfx::Encoder<R, C>,
    device: D,
    pso: gfx::PipelineState<R, shader::pipe::Meta>,
    data: shader::pipe::Data<R>,
}

#[derive(SystemData)]
pub struct Data<'a, R: gfx::Resources> {
    drawable: specs::ReadStorage<'a, shader::Drawable<R>>,
    window: specs::Fetch<'a, window::Window>,
    camera: specs::Fetch<'a, camera::Camera>,
    // TODO: Remove this when proper graphics are added
    space: specs::ReadStorage<'a, ::world::Spatial>,
}

impl<F, C, R, D> System<F, C, R, D>
    where F: gfx::Factory<R>,
          C: gfx::CommandBuffer<R>,
          R: gfx::Resources,
          D: gfx::Device<Resources = R, CommandBuffer = C>
{
    pub fn new(mut factory: F,
               device: D,
               out_color: OutColor<R>,
               out_depth: OutDepth<R>,
               encoder: gfx::Encoder<R, C>,
               pso: gfx::PipelineState<R, shader::pipe::Meta>)
               -> Self {

        let vbuf = factory.create_vertex_buffer(&[]);

        let texels = [[0x0; 4]];
        let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
            texture::Kind::D2(1, 1, texture::AaMode::Single), &[&texels]
        ).unwrap();

        let sampler_info = texture::SamplerInfo::new(texture::FilterMethod::Bilinear,
                                                     texture::WrapMode::Clamp);

        let data = shader::pipe::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            texture: (texture_view, factory.create_sampler(sampler_info)),
            out_color: out_color,
            out_depth: out_depth,
        };

        Self {
            factory,
            device,
            pso,
            data,
            encoder,
        }
    }
}

impl<'a, F, C, R, D> specs::System<'a> for System<F, C, R, D>
    where F: gfx::Factory<R>,
          C: gfx::CommandBuffer<R>,
          R: gfx::Resources,
          D: gfx::Device<Resources = R, CommandBuffer = C>
{
    type SystemData = Data<'a, R>;

    fn run(&mut self, data: Self::SystemData) {
        self.encoder.clear(&self.data.out_color, CLEAR_COLOR);
        self.encoder.clear_depth(&self.data.out_depth, 1.0);

        let mut locals = shader::Locals {
            transform: data.camera.matrix(),
            position: [0.0; 4],
        };

        for (d, s) in (&data.drawable, &data.space).join() {
            let pos = s.position;
            let pos = [pos.x as f32, pos.y as f32, pos.z as f32, 0.0];
            locals.position = pos;

            self.encoder
                .update_constant_buffer(&self.data.locals, &locals);

            self.data.texture.0 = d.texture().clone();
            self.data.vbuf = d.vertex_buffer().clone();
            self.encoder.draw(d.slice(), &self.pso, &self.data);
        }

        self.encoder.flush(&mut self.device);
        data.window.swap_buffers().expect("Failed to swap buffers");
        self.device.cleanup();
    }
}
