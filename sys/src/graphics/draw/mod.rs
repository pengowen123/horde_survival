//! Rendering system
//!
//! Draws each entity that has a `Drawable` component and handles displaying the results to the
//! window.

mod pipeline;
mod components;
mod param;
mod init;
mod utils;

pub use self::init::init;

use self::pipeline::{main, postprocessing};

// TODO: Remove these re-exports when higher-level functionality is exposed
pub use self::pipeline::main::Vertex;
pub use self::pipeline::{ColorFormat, DepthFormat};
//pub use self::pipeline::pipe::init_all as init_pipelines;
pub use self::components::Drawable;
pub use self::param::ShaderParam;

use gfx::{self, texture};
use gfx::traits::FactoryExt;
use glutin::{Window, GlContext};
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
    pso: gfx::PipelineState<R, main::pipe::Meta>,
    pso_post: gfx::PipelineState<R, postprocessing::pipe::Meta>,
    data: main::pipe::Data<R>,
    data_post: postprocessing::pipe::Data<R>,
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
        window: &Window,
        device: D,
        out_color: OutColor<R>,
        out_depth: OutDepth<R>,
        encoder: gfx::Encoder<R, C>,
        pso_main: gfx::PipelineState<R, main::pipe::Meta>,
        pso_post: gfx::PipelineState<R, postprocessing::pipe::Meta>,
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

        // Create texture sampler info
        let sampler_info =
            texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Clamp);

        // Create a render target (postprocessing gets the screen's render target)
        let (width, height) = window.get_inner_size_pixels().expect(
            "Failed to get window size",
        );
        let (width, height) = (width as u16, height as u16);
        let (_, srv, rtv) = factory.create_render_target(width, height).expect(
            "Failed to create render target",
        );
        // Create a depth stencil for the render target
        let dsv = factory
            .create_depth_stencil_view_only(width, height)
            .expect("Failed to create depth stencil");

        let data = main::pipe::Data {
            vbuf: vbuf.clone(),
            locals: factory.create_constant_buffer(1),
            material: factory.create_constant_buffer(1),
            light: factory.create_constant_buffer(1),
            texture: (texture_view.clone(), factory.create_sampler(sampler_info)),
            texture_diffuse: (texture_view.clone(), factory.create_sampler(sampler_info)),
            texture_specular: (texture_view, factory.create_sampler(sampler_info)),
            out_color: rtv,
            out_depth: dsv,
        };

        // TODO: make screen quad here
        let vertices = utils::create_screen_quad();
        let vbuf_post = factory.create_vertex_buffer(&vertices);

        let data_post = postprocessing::pipe::Data {
            vbuf: vbuf_post,
            texture: (srv, factory.create_sampler(sampler_info)),
            screen_color: out_color,
            screen_depth: out_depth,
        };

        Self {
            factory,
            device,
            pso: pso_main,
            pso_post,
            data,
            data_post,
            encoder,
            time: 0.0,
        }
    }

    pub fn factory(&self) -> &F {
        &self.factory
    }

    fn clear_render_targets(&mut self) {
        // Main render target
        self.encoder.clear(&self.data.out_color, CLEAR_COLOR);
        self.encoder.clear_depth(&self.data.out_depth, 1.0);

        // Window render target
        self.encoder.clear(
            &self.data_post.screen_color,
            CLEAR_COLOR,
        );
        self.encoder.clear_depth(&self.data_post.screen_depth, 1.0);
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
        // Clear all render targets
        self.clear_render_targets();

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
        let mut locals = main::Locals {
            mvp: vp.into(),
            model: Matrix4::identity().into(),
            eye_pos,
        };

        let material = main::Material::new(
            // Shininess
            32.0,
        );

        let light = main::Light::new(
            // Position
            [vec[0] as f32, vec[1] as f32, vec[2] as f32, 1.0],
            // Ambient
            [0.01, 0.01, 0.01, 1.0],
            // Diffuse
            [1.0, 1.0, 1.0, 1.0],
            // Specular
            [0.5, 0.5, 0.5, 1.0],
            // Attenuation constant, linear, and quadratic values
            1.0,
            0.09,
            0.032,
        );

        // Draw each entity
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
            // Update the diffuse texture
            self.data.texture_diffuse.0 = d.diffuse().clone();
            // Update the specular texture
            self.data.texture_specular.0 = d.specular().clone();
            // Update the vertex buffer
            self.data.vbuf = d.vertex_buffer().clone();

            // Draw the model
            self.encoder.draw(d.slice(), &self.pso, &self.data);
        }

        // The above code only draws to a texture. This runs postprocessing shaders that draw a
        // screen quad with the texture.
        let slice = gfx::Slice::new_match_vertex_buffer(&self.data_post.vbuf);
        self.encoder.draw(&slice, &self.pso_post, &self.data_post);

        // Send commands to the GPU (actually draw the things)
        self.encoder.flush(&mut self.device);
        // Display the results to the window
        data.window.swap_buffers().expect("Failed to swap buffers");
        // Cleanup resources
        self.device.cleanup();
    }
}
