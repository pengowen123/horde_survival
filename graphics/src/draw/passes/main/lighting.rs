//! Declaration of the lighting pass pipeline
//!
//! Uses the data in the geometry buffer to calculate lighting.

use assets;
use cgmath::{self, SquareMatrix};
use common::config;
use gfx::traits::FactoryExt;
use gfx::{self, format, handle, state, texture};
use rendergraph::error::{BuildError, RunError};
use rendergraph::framebuffer::Framebuffers;
use rendergraph::pass::Pass;
use shred::Resources;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{gbuffer, geometry_pass};
use camera::Camera;
use draw::glsl::{vec4, Mat4, Vec2, Vec3, Vec4};
use draw::passes::{resource_pass, shadow};
use draw::{components, lighting_data, passes, types, utils};

// These constants are inserted into the shaders at runtime; changes here will affect the shaders as
// well
pub const MAX_DIR_LIGHTS: usize = 4;
pub const MAX_POINT_LIGHTS: usize = 4;
pub const MAX_SPOT_LIGHTS: usize = 4;

gfx_defines! {
    vertex Vertex {
        pos: Vec2 = "a_Pos",
        uv: Vec2 = "a_Uv",
    }

    constant Material {
        shininess: f32 = "u_Material_shininess",
    }

    #[derive(Default)]
    constant DirectionalLight {
        direction: Vec4 = "direction",

        ambient: Vec4 = "ambient",
        diffuse: Vec4 = "diffuse",
        specular: Vec4 = "specular",

        has_shadows: f32 = "has_shadows",
        enabled: f32 = "enabled",

        _padding: Vec2 = "_padding",
    }

    #[derive(Default)]
    constant PointLight {
        position: Vec4 = "position",

        ambient: Vec4 = "ambient",
        diffuse: Vec4 = "diffuse",
        specular: Vec4 = "specular",

        constant: f32 = "constant",
        linear: f32 = "linear",
        quadratic: f32 = "quadratic",

        enabled: f32 = "enabled",
    }

    #[derive(Default)]
    constant SpotLight {
        position: Vec4 = "position",
        direction: Vec4 = "direction",

        ambient: Vec4 = "ambient",
        diffuse: Vec4 = "diffuse",
        specular: Vec4 = "specular",

        constant: f32 = "constant",
        linear: f32 = "linear",
        quadratic: f32 = "quadratic",

        cos_cutoff: f32 = "cutOff",
        cos_outer_cutoff: f32 = "outerCutOff",
        enabled: f32 = "enabled",
        _padding: Vec2 = "_padding",
    }

    constant Locals {
        eye_pos: Vec4 = "u_EyePos",
        dir_light_space_matrix: Mat4 = "u_DirLightSpaceMatrix",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "u_Locals",
        material: gfx::ConstantBuffer<Material> = "u_Material",
        // Shadow maps
        dir_shadow_map: gfx::TextureSampler<Vec4> = "t_DirShadowMap",
        // Light buffers
        dir_lights: gfx::ConstantBuffer<DirectionalLight> = "u_DirLights",
        point_lights: gfx::ConstantBuffer<PointLight> = "u_PointLights",
        spot_lights: gfx::ConstantBuffer<SpotLight> = "u_SpotLights",
        // G-buffer textures
        g_position: gfx::TextureSampler<Vec4> = "t_Position",
        g_normal: gfx::TextureSampler<Vec4> = "t_Normal",
        g_color: gfx::TextureSampler<Vec4> = "t_Color",
        // Targets
        out_color: gfx::RenderTarget<format::Rgba8> = "Target0",
        // NOTE: This is `LESS_EQUAL_TEST` instead of `LESS_EQUAL_WRITE`
        depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_TEST,
    }
}

impl Vertex {
    pub fn new(pos: Vec2, uv: Vec2) -> Self {
        Self { pos, uv }
    }
}

impl Material {
    pub fn new(shininess: f32) -> Self {
        Self { shininess }
    }
}

impl DirectionalLight {
    pub fn from_components(
        light: components::DirectionalLight,
        direction: Vec3,
        has_shadows: bool,
    ) -> Self {
        Self {
            direction: vec4(direction, 0.0),
            ambient: light.color.ambient,
            diffuse: light.color.diffuse,
            specular: light.color.specular,
            enabled: 1.0,
            has_shadows: has_shadows as i32 as f32,
            _padding: Default::default(),
        }
    }
}

impl PointLight {
    pub fn from_components(light: components::PointLight, position: Vec3) -> Self {
        Self {
            position: vec4(position, 1.0),
            ambient: light.color.ambient,
            diffuse: light.color.diffuse,
            specular: light.color.specular,
            constant: light.attenuation.constant,
            linear: light.attenuation.linear,
            quadratic: light.attenuation.quadratic,
            enabled: 1.0,
        }
    }
}

impl SpotLight {
    pub fn from_components(light: components::SpotLight, position: Vec3, direction: Vec3) -> Self {
        Self {
            position: vec4(position, 1.0),
            direction: vec4(direction, 0.0),
            ambient: light.color.ambient,
            diffuse: light.color.diffuse,
            specular: light.color.specular,
            constant: light.attenuation.constant,
            linear: light.attenuation.linear,
            quadratic: light.attenuation.quadratic,
            cos_cutoff: light.cos_cutoff().0,
            cos_outer_cutoff: light.cos_outer_cutoff().0,
            enabled: 1.0,
            _padding: Default::default(),
        }
    }
}

pub struct LightingPass<R: gfx::Resources> {
    bundle: gfx::Bundle<R, pipe::Data<R>>,
    shadows: bool,
    shadow_map_size: texture::Size,
}

impl<R: gfx::Resources> LightingPass<R> {
    fn new<F: gfx::Factory<R>>(
        factory: &mut F,
        assets: &assets::Assets,
        gbuffer: &gbuffer::GeometryBuffer<R>,
        rtv: handle::RenderTargetView<R, format::Rgba8>,
        dsv: handle::DepthStencilView<R, types::DepthFormat>,
        dir_shadow_map: handle::ShaderResourceView<R, [f32; 4]>,
        shadows_enabled: bool,
        shadow_map_size: texture::Size,
    ) -> Result<Self, BuildError<String>>
    where
        F: gfx::Factory<R>,
    {
        let pso = Self::load_pso(factory, assets, shadows_enabled)?;

        // Create a screen quad
        let vertices = utils::create_screen_quad(|pos, uv| Vertex::new(pos, uv));
        let vbuf = factory.create_vertex_buffer(&vertices);

        // Create texture sampler info
        let sampler_info =
            texture::SamplerInfo::new(texture::FilterMethod::Bilinear, texture::WrapMode::Tile);

        let shadow_sampler_info = texture::SamplerInfo {
            border: texture::PackedColor::from([1.0; 4]),
            ..texture::SamplerInfo::new(texture::FilterMethod::Scale, texture::WrapMode::Border)
        };

        let srv_pos = gbuffer.position.srv().clone();
        let srv_normal = gbuffer.normal.srv().clone();
        let srv_color = gbuffer.color.srv().clone();

        let data = pipe::Data {
            vbuf: vbuf,
            material: factory.create_constant_buffer(1),
            locals: factory.create_constant_buffer(1),
            dir_shadow_map: (dir_shadow_map, factory.create_sampler(shadow_sampler_info)),
            dir_lights: factory.create_constant_buffer(MAX_DIR_LIGHTS),
            point_lights: factory.create_constant_buffer(MAX_POINT_LIGHTS),
            spot_lights: factory.create_constant_buffer(MAX_SPOT_LIGHTS),
            g_position: (srv_pos, factory.create_sampler(sampler_info)),
            g_normal: (srv_normal, factory.create_sampler(sampler_info)),
            g_color: (srv_color, factory.create_sampler(sampler_info)),
            out_color: rtv.clone(),
            depth: dsv.clone(),
        };

        let slice = gfx::Slice::new_match_vertex_buffer(&data.vbuf);

        let pass = LightingPass {
            bundle: gfx::Bundle::new(slice, pso, data),
            shadows: shadows_enabled,
            shadow_map_size,
        };

        Ok(pass)
    }

    fn load_pso<F: gfx::Factory<R>>(
        factory: &mut F,
        assets: &assets::Assets,
        shadows_enabled: bool,
    ) -> Result<gfx::PipelineState<R, pipe::Meta>, BuildError<String>> {
        let mut defines = HashMap::new();

        defines.insert("MAX_DIR_LIGHTS".into(), MAX_DIR_LIGHTS.to_string());
        defines.insert("MAX_POINT_LIGHTS".into(), MAX_POINT_LIGHTS.to_string());
        defines.insert("MAX_SPOT_LIGHTS".into(), MAX_SPOT_LIGHTS.to_string());

        if shadows_enabled {
            defines.insert("SHADOWS_ENABLED".into(), "1".into());
        }

        passes::load_pso(
            assets,
            factory,
            "lighting_vertex.glsl",
            "lighting_fragment.glsl",
            gfx::Primitive::TriangleList,
            state::Rasterizer::new_fill(),
            pipe::new(),
            defines,
        )
    }
}

pub fn setup_pass<R, C, F>(
    builder: &mut types::GraphBuilder<R, C, F>,
) -> Result<(), BuildError<String>>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    F: gfx::Factory<R>,
{
    let gbuffer = {
        let gbuffer = builder
            .get_pass_output::<geometry_pass::Output<R>>("gbuffer")?
            .gbuffer
            .clone();

        gbuffer
    };
    let (rtv, dsv) = {
        let target = builder
            .get_pass_output::<resource_pass::IntermediateTarget<R>>("intermediate_target")?;
        (target.rtv.clone(), target.dsv.clone())
    };

    let dir_shadow_map = {
        let srv = builder
            .get_pass_output::<shadow::directional::Output<R>>("dir_shadow_map")?
            .srv
            .clone();

        srv
    };

    let (shadows_enabled, shadow_map_size) = {
        let config = builder.get_resources().fetch::<config::GraphicsConfig>();
        (config.shadows, config.shadow_map_size)
    };

    let pass = LightingPass::new(
        builder.factory,
        builder.assets,
        &gbuffer,
        rtv,
        dsv,
        dir_shadow_map,
        shadows_enabled,
        shadow_map_size,
    )?;

    builder.add_pass(pass);

    Ok(())
}

impl<R, C, F> Pass<R, C, F, types::ColorFormat, types::DepthFormat> for LightingPass<R>
where
    R: gfx::Resources,
    C: gfx::CommandBuffer<R>,
    F: gfx::Factory<R>,
{
    fn name(&self) -> &str {
        "lighting"
    }

    fn execute_pass(
        &mut self,
        encoder: &mut gfx::Encoder<R, C>,
        resources: &mut Resources,
    ) -> Result<(), RunError> {
        let camera = resources.fetch::<Arc<Mutex<Camera>>>();
        let lighting_data = resources.fetch::<Arc<Mutex<lighting_data::LightingData>>>();
        let mut lighting_data = lighting_data.lock().unwrap();
        let dir_light_space_matrix = resources.fetch::<Arc<Mutex<shadow::DirShadowSource>>>();

        let dir_light_space_matrix = dir_light_space_matrix
            .lock()
            .unwrap()
            .light_space_matrix()
            .unwrap_or(cgmath::Matrix4::identity());

        // Get camera position
        let eye_pos: [f32; 3] = camera.lock().unwrap().eye_position().into();
        let eye_pos = vec4(eye_pos, 1.0);

        let locals = Locals {
            eye_pos,
            dir_light_space_matrix: dir_light_space_matrix.into(),
        };

        let material = Material::new(32.0);

        encoder.update_constant_buffer(&self.bundle.data.locals, &locals);
        encoder.update_constant_buffer(&self.bundle.data.material, &material);

        // Update light buffers
        let dir_lights = lighting_data.take_dir_lights().collect::<Vec<_>>();
        let point_lights = lighting_data.take_point_lights().collect::<Vec<_>>();
        let spot_lights = lighting_data.take_spot_lights().collect::<Vec<_>>();

        encoder.update_buffer(&self.bundle.data.dir_lights, &dir_lights, 0)?;
        encoder.update_buffer(&self.bundle.data.point_lights, &point_lights, 0)?;
        encoder.update_buffer(&self.bundle.data.spot_lights, &spot_lights, 0)?;

        // Calculate lighting
        self.bundle.encode(encoder);

        Ok(())
    }

    fn reload_shaders(
        &mut self,
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<(), BuildError<String>> {
        self.bundle.pso = Self::load_pso(factory, assets, self.shadows)?;

        Ok(())
    }

    fn handle_window_resize(
        &mut self,
        _: (u16, u16),
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        _: &mut F,
    ) -> Result<(), BuildError<String>> {
        let (rtv, dsv) = {
            let intermediate_target = framebuffers
                .get_framebuffer::<resource_pass::IntermediateTarget<R>>("intermediate_target")?;

            (
                intermediate_target.rtv.clone(),
                intermediate_target.dsv.clone(),
            )
        };

        let gbuffer = framebuffers.get_framebuffer::<gbuffer::GeometryBuffer<R>>("gbuffer")?;

        // Update shader inputs to the resized geometry buffer textures
        self.bundle.data.g_position.0 = gbuffer.position.srv().clone();
        self.bundle.data.g_normal.0 = gbuffer.normal.srv().clone();
        self.bundle.data.g_color.0 = gbuffer.color.srv().clone();

        // Update shader outputs to the resized targets
        self.bundle.data.out_color = rtv;
        self.bundle.data.depth = dsv;

        Ok(())
    }

    fn apply_config(
        &mut self,
        config: &config::GraphicsConfig,
        framebuffers: &mut Framebuffers<R, types::ColorFormat, types::DepthFormat>,
        factory: &mut F,
        assets: &assets::Assets,
    ) -> Result<(), BuildError<String>> {
        let mut update_shadow_map = false;
        // If the shadows setting was changed, reload the shadow map (will be a dummy texture if
        // shadows were disabled) and reload the shaders with the new shadows setting applied
        if config.shadows != self.shadows {
            self.shadows = config.shadows;

            update_shadow_map = true;

            Pass::<R, C, F, _, _>::reload_shaders(self, factory, assets)?;
        }

        // If the shadow map size setting was changed and shadows are enabled, reload the resized
        // shadow map
        if (config.shadow_map_size != self.shadow_map_size) && config.shadows {
            self.shadow_map_size = config.shadow_map_size;

            update_shadow_map = true;
        }

        if update_shadow_map {
            self.bundle.data.dir_shadow_map.0 = framebuffers
                .get_framebuffer::<shadow::directional::Output<R>>("dir_shadow_map")?
                .srv
                .clone();
        }

        Ok(())
    }
}
