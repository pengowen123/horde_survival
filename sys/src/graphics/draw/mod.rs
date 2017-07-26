//! Rendering system
//!
//! Draws each entity that has a `Drawable` component and handles displaying the results to the
//! window.

mod pipeline;
mod components;
mod param;
mod init;

pub use self::init::init;

// TODO: Remove these re-exports when higher-level functionality is exposed
pub use self::pipeline::{Vertex, ColorFormat, DepthFormat};
pub use self::pipeline::pipe::new as init_pipeline;
pub use self::components::Drawable;
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
pub type OutColor<R> = gfx::handle::RenderTargetView<R, pipeline::ColorFormat>;
pub type OutDepth<R> = gfx::handle::DepthStencilView<R, pipeline::DepthFormat>;

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
    pso: gfx::PipelineState<R, pipeline::pipe::Meta>,
    data: pipeline::pipe::Data<R>,
    // TODO: remove, this is for testing lighting
    time: f64,
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
        pso: gfx::PipelineState<R, pipeline::pipe::Meta>,
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

        let data = pipeline::pipe::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            material: factory.create_constant_buffer(1),
            light: factory.create_constant_buffer(1),
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
            time: 0.0,
        }
    }

    pub fn factory(&self) -> &F {
        &self.factory
    }
}

#[derive(SystemData)]
pub struct Data<'a, R: gfx::Resources> {
    drawable: specs::ReadStorage<'a, components::Drawable<R>>,
    window: specs::Fetch<'a, window::Window>,
    camera: specs::Fetch<'a, camera::Camera>,
    // TODO: remove this too
    delta: specs::Fetch<'a, ::delta::Delta>,
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

        // TODO: remove this too
        use cgmath::InnerSpace;
        self.time += data.delta.to_float();
        let x = self.time.sin() * 5.0;
        let y = self.time.cos() * 5.0;
        let vec = ::cgmath::vec3(x, y, 5.0).normalize() * 5.0;

        // Initialize shader uniforms
        let mut locals = pipeline::Locals {
            mvp: vp.into(),
            model: Matrix4::identity().into(),
            eye_pos,
        };

        let material = pipeline::Material::new(
            // Ambient
            [0.2, 0.2, 0.2],
            // Diffuse
            [1.0, 1.0, 1.0],
            // Specular
            [0.5, 0.5, 0.5],
            // Shininess
            32.0,
        );

        let light = pipeline::Light::new(
            // Position
            vec.cast().into(),
            // Ambient
            [0.1, 0.1, 0.1],
            // Diffuse
            [1.0, 1.0, 1.0],
            // Specular
            [0.5, 0.5, 0.5],
        );

        for d in (&data.drawable).join() {
            let param = d.param();
            // Update shader parameters
            let m = param.translation() * param.rotation();
            let mvp = vp * m;
            // Update MVP matrix
            locals.mvp = mvp.into();
            // Update Model matrix
            locals.model = m.into();

            // Update buffers
            self.encoder.update_constant_buffer(
                &self.data.locals,
                &locals,
            );
            self.encoder.update_constant_buffer(
                &self.data.material,
                &material,
            );
            self.encoder.update_constant_buffer(
                &self.data.light,
                &light,
            );
            // Update the texture
            self.data.texture.0 = d.texture().clone();
            // Update the vertex buffer
            self.data.vbuf = d.vertex_buffer().clone();

            // Draw the model
            self.encoder.draw(d.slice(), &self.pso, &self.data);
        }

        // Send commands to the GPU (actually draw the things)
        self.encoder.flush(&mut self.device);
        // Display the results to the window
        data.window.swap_buffers().expect("Failed to swap buffers");
        // Cleanup resources
        self.device.cleanup();
    }
}
