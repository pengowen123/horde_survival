//! Rendering system
//!
//! Draws each entity that has a `Drawable` component and handles displaying the results to the
//! window.

mod shader;
mod param;
mod init;

pub use self::init::init;

// TODO: Remove these re-exports when higher-level functionality is exposed
pub use self::shader::{Vertex, ColorFormat, DepthFormat, Drawable};
pub use self::shader::pipe::new as init_pipeline;
pub use self::param::ShaderParam;

use gfx::{self, texture};
use gfx::traits::FactoryExt;
use glutin::GlContext;
use specs::{self, Join};
use cgmath::{Matrix4, SquareMatrix};

use graphics::camera;
use window;

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

// TODO: Write docs for these type aliases when I figure out what they do
pub type OutColor<R> = gfx::handle::RenderTargetView<R, shader::ColorFormat>;
pub type OutDepth<R> = gfx::handle::DepthStencilView<R, shader::DepthFormat>;

pub struct System<F, C, R, D>
where
    F: gfx::Factory<R>,
    C: gfx::CommandBuffer<R>,
    R: gfx::Resources,
    D: gfx::Device<Resources = R, CommandBuffer = C>,
{
    factory: F,
    encoder: gfx::Encoder<R, C>,
    device: D,
    pso: gfx::PipelineState<R, shader::pipe::Meta>,
    data: shader::pipe::Data<R>,
}

impl<F, C, R, D> System<F, C, R, D>
where
    F: gfx::Factory<R>,
    C: gfx::CommandBuffer<R>,
    R: gfx::Resources,
    D: gfx::Device<Resources = R, CommandBuffer = C>,
{
    pub fn new(
        mut factory: F,
        device: D,
        out_color: OutColor<R>,
        out_depth: OutDepth<R>,
        encoder: gfx::Encoder<R, C>,
        pso: gfx::PipelineState<R, shader::pipe::Meta>,
    ) -> Self {

        // Create dummy data to initialize the shader data

        let vbuf = factory.create_vertex_buffer(&[]);

        let texels = [[0x0; 4]];
        let (_, texture_view) = factory
            .create_texture_immutable::<gfx::format::Rgba8>(
                texture::Kind::D2(1, 1, texture::AaMode::Single),
                &[&texels],
            )
            .unwrap();

        // Create a texture sampler
        let sampler_info =
            texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Clamp);
        let sampler = factory.create_sampler(sampler_info);

        let data = shader::pipe::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            texture: (texture_view, sampler),
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

    pub fn factory(&self) -> &F {
        &self.factory
    }
}

#[derive(SystemData)]
pub struct Data<'a, R: gfx::Resources> {
    drawable: specs::ReadStorage<'a, shader::Drawable<R>>,
    window: specs::Fetch<'a, window::Window>,
    camera: specs::Fetch<'a, camera::Camera>,
}

impl<'a, F, C, R, D> specs::System<'a> for System<F, C, R, D>
where
    F: gfx::Factory<R>,
    C: gfx::CommandBuffer<R>,
    R: gfx::Resources,
    D: gfx::Device<
        Resources = R,
        CommandBuffer = C,
    >,
{
    type SystemData = Data<'a, R>;

    fn run(&mut self, data: Self::SystemData) {
        // Clear the window
        self.encoder.clear(&self.data.out_color, CLEAR_COLOR);
        self.encoder.clear_depth(&self.data.out_depth, 1.0);

        // Get camera matrices
        let proj = data.camera.projection();
        let view = data.camera.view();
        // Proj * View
        let vp = proj * view;

        // Get camera position
        let eye_pos: [f32; 3] = data.camera.eye_position().clone().into();
        let eye_pos = [eye_pos[0], eye_pos[1], eye_pos[2], 1.0];

        // Initialize shader uniforms
        let mut locals = shader::Locals {
            mvp: vp.into(),
            model_view: view.clone().into(),
            model: Matrix4::identity().into(),

            ambient_color: [1.0, 1.0, 1.0, 1.0],
            ambient_strength: 0.05,

            light_pos: [5.0, 5.0, 5.0, 1.0],
            light_color: [0.3, 0.3, 1.0, 1.0],
            light_strength: 0.5,

            eye_pos,
        };

        for d in (&data.drawable).join() {
            let param = d.param();
            // Update shader parameters
            let mvp = vp * param.translation() * param.rotation();
            locals.mvp = mvp.into();
            self.encoder.update_constant_buffer(
                &self.data.locals,
                &locals,
            );
            self.data.texture.0 = d.texture().clone();
            self.data.vbuf = d.vertex_buffer().clone();

            // Draw the model
            self.encoder.draw(d.slice(), &self.pso, &self.data);
        }

        // Cleanup code
        self.encoder.flush(&mut self.device);
        data.window.swap_buffers().expect("Failed to swap buffers");
        self.device.cleanup();
    }
}
