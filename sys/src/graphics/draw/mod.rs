//! Rendering system
//!
//! Draws each entity that has a `Drawable` component and handles displaying the results to the
//! window.

mod pipeline;
mod components;
mod param;
mod init;
mod utils;
mod types;

pub use self::init::init;

use self::pipeline::{main, postprocessing};

// TODO: Remove these re-exports when higher-level functionality is exposed
pub use self::pipeline::main::Vertex;
pub use self::types::{ColorFormat, DepthFormat};
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

const CLEAR_COLOR: [f32; 4] = [1.0; 4];

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
    // Shader pipelines
    pipe_main: main::Pipeline<R>,
    pipe_post: postprocessing::Pipeline<R>,
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
        out_color: types::RenderTargetView<R>,
        encoder: gfx::Encoder<R, C>,
    ) -> Self {

        let (width, height) = window.get_inner_size_pixels().expect(
            "Failed to get window size",
        );
        let (width, height) = (width as u16, height as u16);
        // Create a render target for the main shaders to draw to
        // Postprocessing will read from this render target as a textur
        let (_, srv, rtv) = factory.create_render_target(width, height).expect(
            "Failed to create render target",
        );
        // Create a depth stencil for the render target
        let dsv = factory
            .create_depth_stencil_view_only(width, height)
            .expect("Failed to create depth stencil");


        let main_vs_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/shaders/vertex_150.glsl"
        );
        let main_fs_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/shaders/fragment_150.glsl"
        );
        let pso_main =
            pipeline::load_pso(&mut factory, main_vs_path, main_fs_path, main::pipe::new())
                .unwrap_or_else(|e| panic!("Failed to create main PSO: {}", e));

        let post_vs_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/shaders/post_vertex_150.glsl"
        );
        let post_fs_path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/shaders/post_fragment_150.glsl"
        );

        let pipe_main =
            pipeline::Pipeline::new_main(&mut factory, rtv, dsv, main_vs_path, main_fs_path)
                .unwrap_or_else(|e| panic!("Failed to create main PSO: {}", e));
        let pipe_post =
            pipeline::Pipeline::new_post(&mut factory, srv, out_color, post_vs_path, post_fs_path)
                .unwrap_or_else(|e| panic!("Failed to create postprocessing PSO: {}", e));

        Self {
            factory,
            device,
            pipe_main,
            pipe_post,
            encoder,
        }
    }

    pub fn factory(&self) -> &F {
        &self.factory
    }

    fn clear_render_targets(&mut self) {
        // Main render target
        self.encoder.clear(
            &self.pipe_main.data.out_color,
            CLEAR_COLOR,
        );
        self.encoder.clear_depth(
            &self.pipe_main.data.out_depth,
            1.0,
        );

        // Window render target
        self.encoder.clear(
            &self.pipe_post.data.screen_color,
            CLEAR_COLOR,
        );
    }
}

#[derive(SystemData)]
pub struct Data<'a, R: gfx::Resources> {
    drawable: specs::ReadStorage<'a, components::Drawable<R>>,
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
            [0.0, 0.0, 10.0, 1.0],
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

            let data = &mut self.pipe_main.data;

            // Update buffers
            self.encoder.update_constant_buffer(&data.locals, &locals);
            self.encoder.update_constant_buffer(
                &data.material,
                &material,
            );
            self.encoder.update_constant_buffer(&data.light, &light);
            // Update the texture
            data.texture.0 = d.texture().clone();
            // Update the diffuse texture
            data.texture_diffuse.0 = d.diffuse().clone();
            // Update the specular texture
            data.texture_specular.0 = d.specular().clone();
            // Update the vertex buffer
            data.vbuf = d.vertex_buffer().clone();

            // Draw the model
            self.encoder.draw(d.slice(), &self.pipe_main.pso, data);
        }

        // The above code only draws to a texture. This runs postprocessing shaders that draw a
        // screen quad with the texture.
        let slice = gfx::Slice::new_match_vertex_buffer(&self.pipe_post.data.vbuf);
        self.encoder.draw(
            &slice,
            &self.pipe_post.pso,
            &self.pipe_post.data,
        );

        // Send commands to the GPU (actually draw the things)
        self.encoder.flush(&mut self.device);
        // Display the results to the window
        data.window.swap_buffers().expect("Failed to swap buffers");
        // Cleanup resources
        self.device.cleanup();
    }
}
