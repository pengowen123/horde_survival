//! Declaration of the lighting pass pipeline
//!
//! Uses the data in the geometry buffer to calculate lighting.

use gfx::{self, format, handle, state, texture};
use gfx::traits::FactoryExt;
use rendergraph::pass::Pass;
use shred::Resources;
use cgmath::{self, SquareMatrix};
use assets;

use std::sync::{Arc, Mutex};

use draw::{passes, types, components, utils, lighting_data};
use draw::passes::{shadow, resource_pass};
use draw::glsl::{Vec2, Vec3, Vec4, Mat4, vec4};
use camera::Camera;
use super::{geometry_pass, gbuffer};

// TODO: Enforce that these values match up with the shaders through a new feature in the shader
// preprocessor
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
        out_depth: gfx::DepthTarget<types::DepthFormat> = gfx::preset::depth::LESS_EQUAL_TEST,
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
}

impl<R: gfx::Resources> LightingPass<R> {
    fn new<F: gfx::Factory<R>>(
        factory: &mut F,
        gbuffer: &gbuffer::GeometryBuffer<R>,
        rtv: handle::RenderTargetView<R, format::Rgba8>,
        dsv: handle::DepthStencilView<R, types::DepthFormat>,
        dir_shadow_map: handle::ShaderResourceView<R, [f32; 4]>,
    ) -> Result<Self, passes::PassError>
        where F: gfx::Factory<R>,
    {
        let pso = passes::load_pso(
            factory,
            assets::get_shader_path("lighting_vertex"),
            assets::get_shader_path("lighting_fragment"),
            gfx::Primitive::TriangleList,
            state::Rasterizer::new_fill(),
            pipe::new(),
        )?;

        // Create a screen quad
        let vertices = utils::create_screen_quad(|pos, uv| Vertex::new(pos, uv));
        let vbuf = factory.create_vertex_buffer(&vertices);

        // Create texture sampler info
        let sampler_info = texture::SamplerInfo::new(texture::FilterMethod::Bilinear,
                                                     texture::WrapMode::Tile);

        let shadow_sampler_info =  texture::SamplerInfo {
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
            out_depth: dsv.clone(),
        };

        let slice = gfx::Slice::new_match_vertex_buffer(&data.vbuf);

        let pass = LightingPass {
            bundle: gfx::Bundle::new(slice, pso, data),
        };

        Ok(pass)
    }
}

pub fn setup_pass<R, C, F>(builder: &mut types::GraphBuilder<R, C, F>)
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
          F: gfx::Factory<R>,
{
    let gbuffer = {
        let gbuffer = builder
            .get_pass_output::<geometry_pass::Output<R>>("gbuffer")
            .unwrap()
            .gbuffer
            .clone();
        
        gbuffer
    };
    let (rtv, dsv) = {
        let target = builder
            .get_pass_output::<resource_pass::IntermediateTarget<R>>("intermediate_target")
            .unwrap();
        (target.rtv.clone(), target.dsv.clone())
    };

    let dir_shadow_map = {
        let srv = builder
            .get_pass_output::<shadow::directional::Output<R>>("dir_shadow_map")
            .unwrap()
            .srv
            .clone();

        srv
    };

    let pass = {
        let factory = builder.factory();
        LightingPass::new(
            factory,
            &gbuffer,
            rtv,
            dsv,
            dir_shadow_map,
        ).unwrap()
    };

    builder.add_pass(pass);
}

impl<R, C> Pass<R, C> for LightingPass<R>
    where R: gfx::Resources,
          C: gfx::CommandBuffer<R>,
{
    fn execute_pass(&mut self, encoder: &mut gfx::Encoder<R, C>, resources: &mut Resources) {
        let camera = resources.fetch::<Arc<Mutex<Camera>>>(0);
        let lighting_data = resources.fetch::<Arc<Mutex<lighting_data::LightingData>>>(0);
        let mut lighting_data = lighting_data.lock().unwrap();
        let dir_light_space_matrix = resources
            .fetch::<Arc<Mutex<shadow::DirShadowSource>>>(0);
        
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

        let data = &self.bundle.data;

        let material = Material::new(32.0);

        encoder.update_constant_buffer(&data.locals, &locals);
        encoder.update_constant_buffer(&data.material, &material);

        // Update light buffers
        let dir_lights = lighting_data.take_dir_lights().collect::<Vec<_>>();
        let point_lights = lighting_data.take_point_lights().collect::<Vec<_>>();
        let spot_lights = lighting_data.take_spot_lights().collect::<Vec<_>>();

        encoder.update_buffer(&data.dir_lights, &dir_lights, 0).unwrap();
        encoder.update_buffer(&data.point_lights, &point_lights, 0).unwrap();
        encoder.update_buffer(&data.spot_lights, &spot_lights, 0).unwrap();

        // Calculate lighting
        self.bundle.encode(encoder);
    }
}
