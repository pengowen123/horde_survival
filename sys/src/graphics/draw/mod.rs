//! Rendering system
//!
//! Draws each entity that has a `Drawable` component and handles displaying the results to the
//! window.

// TODO: this file is getting a bit large, look into moving some code in new modules
#[macro_use]
mod utils;
pub mod components;
mod pipeline;
mod param;
mod init;
mod types;
mod factory_ext;
mod lighting_data;
mod render_target;
mod glsl;

pub use self::init::init;

// TODO: Remove these re-exports when higher-level functionality is exposed
pub use self::pipeline::main::geometry_pass::Vertex;
pub use self::pipeline::main::lighting::Material;
pub use self::types::{ColorFormat, DepthFormat};
pub use self::components::Drawable;
pub use self::param::ShaderParam;

use gfx::{self, handle, texture};
use glutin::{Window, GlContext};
use specs::{self, Join};
use cgmath::{self, Matrix4, SquareMatrix};

use std::sync::mpsc;

use self::pipeline::{postprocessing, skybox};
use self::pipeline::main::{self, geometry_pass, lighting};
use self::pipeline::shadow;
use self::pipeline::shadow::traits::{LightShadows, AspectRatio};
use graphics::camera;
use assets;
use window;

const CLEAR_COLOR: [f32; 4] = [0.0; 4];

/// A `specs::Storage` for the `Drawable` component
pub type DrawableStorage<'a, R> =
    specs::Storage<
        'a,
        components::Drawable<R>,
        specs::Fetch<'a, specs::MaskedStorage<components::Drawable<R>>>,
    >;

pub struct System<F, C, R, D>
where
    F: gfx::Factory<R>,
    C: gfx::CommandBuffer<R>,
    R: gfx::Resources,
    D: gfx::Device<Resources = R, CommandBuffer = C>,
{
    // GFX structs
    factory: F,
    encoder: gfx::Encoder<R, C>,
    device: D,
    // Off-screen render targets
    render_targets: render_target::RenderTargets<R>,
    // Shader pipelines
    pipe_geometry_pass: geometry_pass::Pipeline<R>,
    pipe_dir_light: lighting::PipelineDirLight<R>,
    pipe_point_light: lighting::PipelinePointLight<R>,
    pipe_spot_light: lighting::PipelineSpotLight<R>,
    pipe_dir_shadow: shadow::directional::Pipeline<R>,
    pipe_point_shadow: shadow::point::Pipeline<R>,
    pipe_post: postprocessing::Pipeline<R>,
    pipe_skybox: skybox::Pipeline<R>,
    aspect_ratio_point: mpsc::Sender<AspectRatio>,
    aspect_ratio_spot: mpsc::Sender<AspectRatio>,
}

impl<F, C, R, D> System<F, C, R, D>
where
    F: gfx::Factory<R>,
    C: gfx::CommandBuffer<R>,
    R: gfx::Resources,
    D: gfx::Device<Resources = R, CommandBuffer = C>,
{
    // TODO: Make this return result so the application can handle the error
    pub fn new(
        mut factory: F,
        window: &Window,
        device: D,
        out_color: handle::RenderTargetView<R, types::ColorFormat>,
        encoder: gfx::Encoder<R, C>,
        aspect_ratios: (mpsc::Sender<AspectRatio>, mpsc::Sender<AspectRatio>),
    ) -> Self {

        // Get the dimensions for all new render targets
        let (width, height) = window.get_inner_size_pixels().expect(
            "Failed to get window size",
        );
        let (width, height) = (width as texture::Size, height as texture::Size);

        // Anti-aliasing mode
        // TODO: learn how to do anti-aliasing with deferred shading
        //let aa_mode = texture::AaMode::Multi(8);

        // Create a geometry buffer
        let gbuffer = main::gbuffer::GeometryBuffer::new(&mut factory, width, height).unwrap();

        // Create off-screen render targets
        let render_targets = render_target::RenderTargets::new(&mut factory, width, height)
            .unwrap();

        let (rtv, srv) = {
            let active = render_targets.get_active();
            let rtv = active.rtv().clone();
            let srv = active.srv().clone();
            (rtv, srv)
        };

        // Geometry pass pipeline
        let pipe_geometry_pass =
            pipeline::Pipeline::new_geometry_pass(
                &mut factory,
                gbuffer.position.rtv().clone(),
                gbuffer.normal.rtv().clone(),
                gbuffer.color.rtv().clone(),
                gbuffer.depth.clone(),
                assets::get_shader_path("geometry_pass_vertex"),
                assets::get_shader_path("geometry_pass_fragment"),
            ).unwrap_or_else(|e| panic!("Failed to create geometry pass PSO: {}", e));

        // Directional light shadow pipeline
        let shadow_map_size = 1024;

        let (pipe_dir_shadow, dir_shadow_map) = pipeline::Pipeline::new_dir_shadow(
            &mut factory,
            shadow_map_size,
            assets::get_shader_path("dir_shadow_vertex"),
            assets::get_shader_path("dir_shadow_fragment"),
        ).unwrap_or_else(|e| {
            panic!("Failed to create directional light shadow PSO: {}", e)
        });

        let (pipe_point_shadow, point_shadow_map) =
            pipeline::Pipeline::new_point_shadow(
                &mut factory,
                shadow_map_size,
                assets::get_shader_path("point_shadow_vertex"),
                assets::get_shader_path("point_shadow_geometry"),
                assets::get_shader_path("point_shadow_fragment"),
            ).unwrap_or_else(|e| panic!("Failed to create point light shadow PSO: {}", e));

        // Directional light pipeline
        let pipe_dir_light =
            pipeline::Pipeline::new_dir_light(
                &mut factory,
                dir_shadow_map,
                gbuffer.position.srv().clone(),
                gbuffer.normal.srv().clone(),
                gbuffer.color.srv().clone(),
                srv.clone(),
                rtv.clone(),
                assets::get_shader_path("light_vertex"),
                assets::get_shader_path("dir_light_fragment"),
            ).unwrap_or_else(|e| panic!("Failed to create directional light PSO: {}", e));

        // Point light pipeline
        let pipe_point_light =
            pipeline::Pipeline::new_point_light(
                &mut factory,
                // TODO: Use a shadow map instead of this srv
                srv.clone(),
                gbuffer.position.srv().clone(),
                gbuffer.normal.srv().clone(),
                gbuffer.color.srv().clone(),
                srv.clone(),
                rtv.clone(),
                assets::get_shader_path("light_vertex"),
                assets::get_shader_path("point_light_fragment"),
            ).unwrap_or_else(|e| panic!("Failed to create point light PSO: {}", e));

        // Spot light pipeline
        let pipe_spot_light =
            pipeline::Pipeline::new_spot_light(
                &mut factory,
                // TODO: Use a shadow map instead of this srv
                srv.clone(),
                gbuffer.position.srv().clone(),
                gbuffer.normal.srv().clone(),
                gbuffer.color.srv().clone(),
                srv.clone(),
                rtv.clone(),
                assets::get_shader_path("light_vertex"),
                assets::get_shader_path("spot_light_fragment"),
            ).unwrap_or_else(|e| panic!("Failed to create spot light PSO: {}", e));

        // Skybox pipeline
        let mut pipe_skybox = pipeline::Pipeline::new_skybox(
            &mut factory,
            rtv.clone(),
            gbuffer.depth,
            point_shadow_map,
            assets::get_shader_path("skybox_vertex"),
            assets::get_shader_path("skybox_fragment"),
        ).unwrap_or_else(|e| panic!("Failed to create skybox PSO: {}", e));

        //pipe_skybox.data.skybox.0 = point_shadow_map;

        // Postprocessing pipeline
        let pipe_post = pipeline::Pipeline::new_post(
            &mut factory,
            srv.clone(),
            out_color,
            assets::get_shader_path("post_vertex"),
            assets::get_shader_path("post_fragment"),
        ).unwrap_or_else(|e| panic!("Failed to create postprocessing PSO: {}", e));

        Self {
            factory,
            encoder,
            device,
            render_targets,
            pipe_geometry_pass,
            pipe_dir_light,
            pipe_point_light,
            pipe_spot_light,
            pipe_dir_shadow,
            pipe_point_shadow,
            pipe_post,
            pipe_skybox,
            aspect_ratio_point: aspect_ratios.0,
            aspect_ratio_spot: aspect_ratios.1,
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
    fn draw_entity_to_gbuffer(
        &mut self,
        drawable: &components::Drawable<R>,
        view_proj: Matrix4<f32>,
        locals: &mut geometry_pass::Locals,
    ) {
        // Get model-specific transform matrix
        let model = drawable.param().get_model_matrix();

        // Update shader parameters
        locals.model = model.into();
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

    /// Uses the data in the geometry buffer to calculate lighting from the provided lights
    ///
    /// To draw shadows for lights that have them enabled, all entities must be redrawn to a shadow
    /// map.
    fn draw_lighting<'a>(
        &mut self,
        drawable: &DrawableStorage<'a, R>,
        lighting_data: &lighting_data::LightingData,
        camera: &camera::Camera,
    ) {
        // Get camera position
        let eye_pos: [f32; 3] = camera.eye_position().into();
        let eye_pos = [eye_pos[0], eye_pos[1], eye_pos[2], 1.0];

        let mut lighting_locals = lighting::Locals {
            eye_pos,
            light_space_matrix: cgmath::Matrix4::identity().into(),
        };
        let material = lighting::Material::new(32.0);

        // NOTE: This slice is shared by all light pipelines, because they all just use screen
        //       quads for their vertex buffers
        let slice = gfx::Slice::new_match_vertex_buffer(&self.pipe_post.data.vbuf);

        let encoder = &mut self.encoder;
        let dir_light = &mut self.pipe_dir_light;
        let dir_shadow = &mut self.pipe_dir_shadow;
        let point_light = &mut self.pipe_point_light;
        let point_shadow = &mut self.pipe_point_shadow;
        let spot_light = &mut self.pipe_spot_light;

        // Update constant buffers for the directional light pipeline
        encoder.update_constant_buffer(&dir_light.data.locals, &lighting_locals);
        encoder.update_constant_buffer(&dir_light.data.material, &material);

        // Update constant buffers for the point light pipeline
        encoder.update_constant_buffer(&point_light.data.locals, &lighting_locals);
        encoder.update_constant_buffer(&point_light.data.material, &material);

        // Update constant buffers for the spot light pipeline
        encoder.update_constant_buffer(&spot_light.data.locals, &lighting_locals);
        encoder.update_constant_buffer(&spot_light.data.material, &material);

        // Draw all directional lights lights
        for l in lighting_data.dir_lights() {
            let light = l.light;
            let shadows = l.shadows;
            let light_space_matrix = l.transform;

            encoder.update_constant_buffer(&dir_light.data.light, &light);

            // Use the active render target
            {
                let active = self.render_targets.get_active();

                dir_light.data.target_color.0 = active.srv().clone();
                dir_light.data.out_color = active.rtv().clone();
            }

            // Clear the shadow map
            encoder.clear_depth(&dir_shadow.data.out_depth, 1.0);
            // Reset the light space matrix
            lighting_locals.light_space_matrix = cgmath::Matrix4::identity().into();

            // Draw shadows if the light has them enabled
            if let components::ShadowSettings::Enabled = shadows {
                lighting_locals.light_space_matrix = light_space_matrix.into();

                components::DirectionalLight::render_shadow_map(
                    drawable,
                    light_space_matrix,
                    |slice, vbuf, locals| {
                        dir_shadow.data.vbuf = vbuf;
                        encoder.update_constant_buffer(&dir_shadow.data.locals, locals);

                        encoder.draw(slice, &dir_shadow.pso, &dir_shadow.data);
                    },
                );
            }

            // Update uniforms for the lighting shader
            encoder.update_constant_buffer(&dir_light.data.locals, &lighting_locals);

            // Draw the light
            encoder.draw(&slice, &dir_light.pso, &dir_light.data);

            // Swap render targets
            self.render_targets.swap_render_targets();
        }

        // Reset the light space matrix
        // It is not used for point lights so it is just the identity matrix
        lighting_locals.light_space_matrix = cgmath::Matrix4::identity().into();

        // Draw all point lights
        for l in lighting_data.point_lights() {
            let light = l.light;
            let shadows = l.shadows;
            let transform = l.transform;

            encoder.update_constant_buffer(&point_light.data.light, &light);

            // Use the active render target
            {
                let active = self.render_targets.get_active();

                point_light.data.target_color.0 = active.srv().clone();
                point_light.data.out_color = active.rtv().clone();
            }

            // Clear the shadow map
            encoder.clear_depth(&dir_shadow.data.out_depth, 1.0);

            if let components::ShadowSettings::Enabled = shadows {
                components::PointLight::render_shadow_map(
                    drawable,
                    transform,
                    |slice, vbuf, locals| {
                        point_shadow.data.vbuf = vbuf;
                        point_shadow.data.light_pos = locals.light_pos;
                        point_shadow.data.far_plane = locals.far_plane;
                        encoder
                            .update_buffer(
                                &point_shadow.data.view_matrices,
                                &locals.view_matrices,
                                0,
                            )
                            .unwrap();

                        encoder.draw(slice, &point_shadow.pso, &point_shadow.data);
                    },
                );
            }

            encoder.draw(&slice, &point_light.pso, &point_light.data);

            // Swap render targets
            self.render_targets.swap_render_targets();
        }

        // Draw all spot lights
        for l in lighting_data.spot_lights() {
            let light = l.light;
            let shadows = l.shadows;

            encoder.update_constant_buffer(&spot_light.data.light, &light);

            // Use the active render target
            {
                let active = self.render_targets.get_active();

                spot_light.data.target_color.0 = active.srv().clone();
                spot_light.data.out_color = active.rtv().clone();
            }

            encoder.draw(&slice, &spot_light.pso, &spot_light.data);

            // Swap render targets
            self.render_targets.swap_render_targets();
        }

        // Swap render targets so the next pipeline gets an unused one
        self.render_targets.swap_render_targets();
    }

    /// Draws the skybox given the `View * Projection` matrix of the camera
    fn draw_skybox(&mut self, camera: Matrix4<f32>) {
        // Use the active render target
        self.pipe_skybox.data.out_color = self.render_targets.get_active().rtv().clone();

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

        // Swap active render targets so the next pipeline gets an unused one
        self.render_targets.swap_render_targets();
    }

    /// Applies postprocessing effects
    ///
    /// This function must be called after all drawing is done so the results will be displayed to
    /// the window.
    fn draw_postprocessing(&mut self) {
        // Use the active render target
        self.pipe_post.data.texture.0 = self.render_targets.get_active().srv().clone();
        //self.pipe_post.data.texture.0 = self.pipe_dir_light.data.shadow_map.0.clone();

        let slice = gfx::Slice::new_match_vertex_buffer(&self.pipe_post.data.vbuf);
        self.encoder.draw(
            &slice,
            &self.pipe_post.pso,
            &self.pipe_post.data,
        );

        // Swap active render_targets so the next pipeline gets an unused one
        self.render_targets.swap_render_targets();
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
        let (target_0, target_1) = self.render_targets.get_all_render_targets();

        // NOTE: Make sure this is kept up to date as new pipelines are added
        clear_targets!(
            COLOR, self,
            // General render targets
            target_0,
            target_1,
            // Geometry buffer
            self.pipe_geometry_pass.data.out_pos,
            self.pipe_geometry_pass.data.out_normal,
            self.pipe_geometry_pass.data.out_color,
            // Misc
            self.pipe_skybox.data.out_color,
            self.pipe_post.data.screen_color,
        );

        clear_targets!(
            DEPTH, self,
            self.pipe_geometry_pass.data.out_depth,
            self.pipe_skybox.data.out_depth,
        );
    }

    fn update_shadow_map_aspect_ratios(&self) {
        self.aspect_ratio_point
            .send(AspectRatio::from_depth_stencil(
                &self.pipe_dir_shadow.data.out_depth,
            ))
            .unwrap();
        // XXX: send spot shadow ratio here
    }

    /// Reloads the shaders
    fn reload_shaders(&mut self) -> Result<(), pipeline::PipelineError> {
        // TODO: remake this with multisampling and other graphics settings applied
        Ok(())
    }
}

#[derive(SystemData)]
pub struct Data<'a, R: gfx::Resources> {
    drawable: specs::ReadStorage<'a, components::Drawable<R>>,
    window: specs::Fetch<'a, window::Window>,
    camera: specs::Fetch<'a, camera::Camera>,
    lighting_data: specs::Fetch<'a, lighting_data::LightingData>,
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

        self.update_shadow_map_aspect_ratios();

        // Clear all render and depth targets
        self.clear_targets();

        // Get camera matrix
        let camera = data.camera;
        let vp = camera.projection() * camera.view();

        // Initialize shader uniforms
        let mut locals = geometry_pass::Locals {
            model: Matrix4::identity().into(),
            view_proj: vp.into(),
        };

        // Draw each entity (to the geometry buffer)
        let entities = (&data.drawable).join();

        for d in entities {
            self.draw_entity_to_gbuffer(d, vp, &mut locals);
        }

        // Apply lighting
        self.draw_lighting(&data.drawable, &data.lighting_data, &camera);

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
