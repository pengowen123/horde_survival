//! Rendering system
//!
//! Draws each entity that has a `Drawable` component and handles displaying the results to the
//! window.

// TODO: this file is getting a bit large, look into moving some code in new modules
pub mod components;
mod pipeline;
mod param;
mod init;
mod types;
mod factory_ext;
#[macro_use]
mod utils;

pub use self::init::init;

// TODO: Remove these re-exports when higher-level functionality is exposed
pub use self::pipeline::main::geometry_pass::Vertex;
pub use self::pipeline::main::lighting::Material;
pub use self::types::{ColorFormat, DepthFormat};
pub use self::components::Drawable;
pub use self::param::ShaderParam;

use gfx::{self, handle, format};
use glutin::{Window, GlContext};
use specs::{self, Join};
use cgmath::{Matrix4, SquareMatrix};

use self::pipeline::{postprocessing, skybox};
use self::pipeline::main::{self, geometry_pass, lighting};
use graphics::camera;
use window;

const CLEAR_COLOR: [f32; 4] = [0.0; 4];

// TODO: replace these with asset loader struct that hands out paths
const GEOMETRY_PASS_VS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/shaders/geometry_pass_vertex_150.glsl"
);
const GEOMETRY_PASS_FS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/shaders/geometry_pass_fragment_150.glsl"
);

const LIGHTING_VS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/shaders/lighting_vertex_150.glsl"
);
const LIGHTING_FS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/shaders/lighting_fragment_150.glsl"
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
    pipe_geometry_pass: geometry_pass::Pipeline<R>,
    pipe_lighting: lighting::Pipeline<R>,
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
        out_color: handle::RenderTargetView<R, types::ColorFormat>,
        encoder: gfx::Encoder<R, C>,
    ) -> Self {

        // Get the dimensions for all new render targets
        let (width, height) = window.get_inner_size_pixels().expect(
            "Failed to get window size",
        );
        let (width, height) = (width as u16, height as u16);

        // Anti-aliasing mode
        // TODO: learn how to do anti-aliasing with deferred shading
        //let aa_mode = texture::AaMode::Multi(8);

        // Create a geometry buffer
        let gbuffer = main::gbuffer::create_geometry_buffer(&mut factory, width, height);

        // Create an intermediate render target (postprocessing uses the window target)
        let (_, srv, rtv) = factory
            .create_render_target::<format::Rgba8>(width, height)
            .unwrap();

        // Geometry pass pipeline
        let pipe_geometry_pass =
            pipeline::Pipeline::new_geometry_pass(
                &mut factory,
                gbuffer.position.target,
                gbuffer.normal.target,
                gbuffer.color.target,
                gbuffer.depth.clone(),
                GEOMETRY_PASS_VS_PATH,
                GEOMETRY_PASS_FS_PATH,
            ).unwrap_or_else(|e| panic!("Failed to create geometry pass PSO: {}", e));

        let pipe_lighting = pipeline::Pipeline::new_lighting(
            &mut factory,
            gbuffer.position.resource,
            gbuffer.normal.resource,
            gbuffer.color.resource,
            rtv.clone(),
            LIGHTING_VS_PATH,
            LIGHTING_FS_PATH,
        ).unwrap_or_else(|e| panic!("Failed to create lighting PSO: {}", e));

        // Skybox pipeline
        let pipe_skybox = pipeline::Pipeline::new_skybox(
            &mut factory,
            rtv,
            gbuffer.depth,
            SKYBOX_VS_PATH,
            SKYBOX_FS_PATH,
        ).unwrap_or_else(|e| panic!("Failed to create skybox PSO: {}", e));

        // Postprocessing pipeline
        let pipe_post =
            pipeline::Pipeline::new_post(&mut factory, srv, out_color, POST_VS_PATH, POST_FS_PATH)
                .unwrap_or_else(|e| panic!("Failed to create postprocessing PSO: {}", e));

        Self {
            factory,
            device,
            pipe_geometry_pass,
            pipe_lighting,
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
    ///
    /// This function will only write data to the geometry buffer. To see the results,
    /// `draw_lighting` must be called afer calling this function.
    fn draw_entity(
        &mut self,
        drawable: &components::Drawable<R>,
        view_proj: Matrix4<f32>,
        locals: &mut geometry_pass::Locals,
    ) {
        // Get model-specific transform matrix
        let param = drawable.param();
        let m = param.translation() * param.rotation() * param.scale();

        // Update shader parameters
        locals.model = m.into();
        locals.view_proj = view_proj.into();

        let data = &mut self.pipe_geometry_pass.data;

        // Update model-specific buffers
        self.encoder.update_constant_buffer(&data.locals, locals);

        // TODO: use the entity's material
        //self.encoder.update_constant_buffer(
        //&data.material,
        //&drawable.material(),
        //);

        // Update texture maps
        data.diffuse.0 = drawable.diffuse().clone();
        data.specular.0 = drawable.specular().clone();

        // Update the vertex buffer
        data.vbuf = drawable.vertex_buffer().clone();

        // Draw the model
        self.encoder.draw(
            drawable.slice(),
            &self.pipe_geometry_pass.pso,
            data,
        );
    }

    /// Uses the data in the geometry buffer to calculate lighting
    fn draw_lighting(&mut self, eye_pos: [f32; 4]) {
        let lighting_locals = lighting::Locals { eye_pos };
        let material = lighting::Material::new(32.0);

        let slice = gfx::Slice::new_match_vertex_buffer(&self.pipe_skybox.data.vbuf);

        self.encoder.update_constant_buffer(
            &self.pipe_lighting.data.locals,
            &lighting_locals,
        );
        self.encoder.update_constant_buffer(
            &self.pipe_lighting.data.material,
            &material,
        );
        self.encoder.draw(
            &slice,
            &self.pipe_lighting.pso,
            &self.pipe_lighting.data,
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
    ///
    /// This function must be called after all drawing is done so the results will be displayed to
    /// the window.
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

    /// Clears all render and depth targets
    fn clear_targets(&mut self) {
        // NOTE: Make sure this is kept up to date as new pipelines are added
        clear_targets!(
            COLOR, self,
            self.pipe_geometry_pass.data.out_pos,
            self.pipe_geometry_pass.data.out_normal,
            self.pipe_geometry_pass.data.out_color,
            self.pipe_lighting.data.out_color,
            self.pipe_skybox.data.out_color,
            self.pipe_post.data.screen_color,
        );

        clear_targets!(
            DEPTH, self,
            self.pipe_geometry_pass.data.out_depth,
            self.pipe_skybox.data.out_depth,
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

        // Clear all render and depth targets
        self.clear_targets();

        // Get camera matrix
        let camera = data.camera;
        let vp = camera.projection() * camera.view();

        // Get camera position
        let eye_pos: [f32; 3] = camera.eye_position().into();
        let eye_pos = [eye_pos[0], eye_pos[1], eye_pos[2], 1.0];

        // Initialize shader uniforms
        let mut locals = geometry_pass::Locals {
            model: Matrix4::identity().into(),
            view_proj: vp.into(),
        };

        // Draw each entity (to the geometry buffer)
        for d in (&data.drawable).join() {
            self.draw_entity(d, vp, &mut locals);
        }

        // Apply lighting
        self.draw_lighting(eye_pos);

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
