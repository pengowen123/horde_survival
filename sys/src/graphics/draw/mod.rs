//! Rendering system
//!
//! Draws each entity that has a `Drawable` component and handles displaying the results to the
//! window.

// TODO: this file is getting a bit large, look into moving some code in new modules
mod pipeline;
mod components;
mod param;
mod init;
mod types;
mod factory_ext;
mod utils;

pub use self::init::init;

use self::pipeline::{main, postprocessing, skybox};

// TODO: Remove these re-exports when higher-level functionality is exposed
pub use self::pipeline::main::Vertex;
pub use self::types::{ColorFormat, DepthFormat};
//pub use self::pipeline::pipe::init_all as init_pipelines;
pub use self::components::Drawable;
pub use self::param::ShaderParam;

use gfx;
use glutin::{Window, GlContext};
use specs::{self, Join};
use cgmath::{Matrix4, SquareMatrix};

use graphics::camera;
use window;

const CLEAR_COLOR: [f32; 4] = [1.0; 4];

// TODO: replace these with asset loader struct that hands out paths
const MAIN_VS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/shaders/vertex_150.glsl"
);

const MAIN_FS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/shaders/fragment_150.glsl"
);

const POST_VS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/shaders/post_vertex_150.glsl"
);
const POST_FS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/shaders/post_fragment_150.glsl"
);

const SKYBOX_VS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/shaders/skybox_vertex_150.glsl"
);
const SKYBOX_FS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/shaders/skybox_fragment_150.glsl"
);

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
    pipe_skybox: skybox::Pipeline<R>,
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
        // Postprocessing will read from this render target as a texture
        let (_, srv, rtv) = factory.create_render_target(width, height).expect(
            "Failed to create render target",
        );
        // Create a depth stencil for the render target
        let dsv = factory
            .create_depth_stencil_view_only(width, height)
            .expect("Failed to create depth stencil");

        let pipe_main = pipeline::Pipeline::new_main(
            &mut factory,
            rtv.clone(),
            dsv.clone(),
            MAIN_VS_PATH,
            MAIN_FS_PATH,
        ).unwrap_or_else(|e| panic!("Failed to create main PSO: {}", e));

        let pipe_post =
            pipeline::Pipeline::new_post(&mut factory, srv, out_color, POST_VS_PATH, POST_FS_PATH)
                .unwrap_or_else(|e| panic!("Failed to create postprocessing PSO: {}", e));

        let pipe_skybox =
            pipeline::Pipeline::new_skybox(&mut factory, rtv, dsv, SKYBOX_VS_PATH, SKYBOX_FS_PATH)
                .unwrap_or_else(|e| panic!("Failed to create skybox PSO: {}", e));

        Self {
            factory,
            device,
            pipe_main,
            pipe_post,
            pipe_skybox,
            encoder,
        }
    }

    pub fn factory(&self) -> &F {
        &self.factory
    }

    /// Draws an entity given its `Drawable` component, a set of shader parameters, a `View *
    /// Projection` matrix, and a constant buffer to write shader input to
    fn draw_entity(
        &mut self,
        drawable: &components::Drawable<R>,
        view_proj: Matrix4<f32>,
        locals: &mut main::Locals,
    ) {
        // Get model-specific transform matrix
        let param = drawable.param();
        let m = param.translation() * param.rotation();
        let mvp = view_proj * m;

        // Update shader parameters
        locals.mvp = mvp.into();
        locals.model = m.into();

        let data = &mut self.pipe_main.data;

        // Update model-specific buffers
        self.encoder.update_constant_buffer(&data.locals, locals);

        // Update texture and lighting maps
        data.texture.0 = drawable.texture().clone();
        data.texture_diffuse.0 = drawable.diffuse().clone();
        data.texture_specular.0 = drawable.specular().clone();

        // Update the vertex buffer
        data.vbuf = drawable.vertex_buffer().clone();

        // Draw the model
        self.encoder.draw(
            drawable.slice(),
            &self.pipe_main.pso,
            data,
        );
    }

    /// Draws the skybox given the `View * Projection` matrix of the camera
    fn draw_skybox(&mut self, camera: Matrix4<f32>) {
        let skybox_locals = skybox::Locals { view_proj: camera.into() };

        let slice = gfx::Slice::new_match_vertex_buffer(&self.pipe_skybox.data.vbuf);
        self.encoder.update_constant_buffer(
            &self.pipe_skybox.data.locals,
            &skybox_locals,
        );
        self.encoder.draw(
            &slice,
            &self.pipe_skybox.pso,
            &self.pipe_skybox.data,
        );

    }

    /// Applies postprocessing effects
    fn draw_postprocessing(&mut self) {
        let slice = gfx::Slice::new_match_vertex_buffer(&self.pipe_post.data.vbuf);
        self.encoder.draw(
            &slice,
            &self.pipe_post.pso,
            &self.pipe_post.data,
        );
    }

    /// Displays everything previously drawn to the window
    fn display(&mut self, window: &window::Window) {
        // Send commands to the GPU (actually draw the things)
        self.encoder.flush(&mut self.device);
        // Display the results to the window
        window.swap_buffers().expect("Failed to swap buffers");
        // Cleanup resources
        self.device.cleanup();
    }

    /// Clears all render targets
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

    /// Reloads the shaders
    fn reload_shaders(&mut self) -> Result<(), pipeline::PsoError> {
        // TODO: remake this with multisampling and other graphics settings applied
        Ok(())
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
        // TODO: make a better way to do this
        //self.reload_shaders().unwrap_or_else(|e| {
        //eprintln!("Failed to reload shaders: {}", e);
        //});

        // Clear all render targets
        self.clear_render_targets();

        // Get camera matrix
        let camera = data.camera;
        let vp = camera.projection() * camera.view();

        // Get camera position
        let eye_pos: [f32; 3] = camera.eye_position().into();
        let eye_pos = [eye_pos[0], eye_pos[1], eye_pos[2], 1.0];

        // Initialize shader uniforms
        let mut locals = main::Locals {
            mvp: vp.into(),
            model: Matrix4::identity().into(),
            eye_pos,
        };

        // TODO: cleanup material and light initialization when deferred is implemented
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

        // Update buffers
        self.encoder.update_constant_buffer(
            &self.pipe_main.data.material,
            &material,
        );
        self.encoder.update_constant_buffer(
            &self.pipe_main.data.light,
            &light,
        );

        // Draw each entity
        for d in (&data.drawable).join() {
            self.draw_entity(d, vp, &mut locals);
        }

        // Draw the skybox
        self.draw_skybox(camera.skybox_camera());

        // Apply postprocessing effects
        //
        // If postprocessing is enabled, the above code will draw to an intermediate texture, which
        // is then used by the postprocessing shaders here
        self.draw_postprocessing();

        // Display the results to the screen
        self.display(&data.window);
    }
}
