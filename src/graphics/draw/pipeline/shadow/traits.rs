//! Traits for working with shadows for different light types

use cgmath::{self, SquareMatrix, Angle};
use gfx::{self, handle};
use specs::Join;

use graphics::draw::{self, components};
use graphics::draw::pipeline::main::{lighting, geometry_pass};
use graphics::draw::pipeline::shadow;

/// The aspect ratio of a render target
#[derive(Clone, Copy, Debug)]
pub struct AspectRatio(pub f32);

impl AspectRatio {
    pub fn from_render_target<R, CF>(rtv: &handle::RenderTargetView<R, CF>) -> Self
    where
        R: gfx::Resources,
    {
        let (width, height, _, _) = rtv.get_dimensions();

        AspectRatio(width as f32 / height as f32)
    }

    pub fn from_depth_stencil<R, DF>(dsv: &handle::DepthStencilView<R, DF>) -> Self
    where
        R: gfx::Resources,
    {
        let (width, height, _, _) = dsv.get_dimensions();

        AspectRatio(width as f32 / height as f32)
    }
}

impl Default for AspectRatio {
    fn default() -> Self {
        // NOTE: This cannot be 0.0 or a panic will happen when creating a projection matrix
        AspectRatio(1.0)
    }
}

/// A trait implemented by light types, used for getting the information needed to render shadow
/// maps for lights of those types
pub trait LightTransform {
    /// The transform data for rendering a shadow map for the light
    type Transform;
    /// Additional data required to calculate transform data for the light
    type RequiredData;
    /// The constant buffer struct that corresponds to the light type
    type ShaderStruct;
    /// Returns the transform data for the light
    fn get_light_space_transform(&self, data: Self::RequiredData) -> Self::Transform;
}

/// A trait implemented by light types, used for rendering shadow maps given transform data
pub trait LightShadows: LightTransform {
    /// The shader uniform type
    type Locals;

    /// Renders each entity with the `Drawable` component to the shadow map, given the transform
    /// data for a light, and a function to draw individual entities
    fn render_shadow_map<R, F>(
        drawable: &draw::DrawableStorage<R>,
        transform: Self::Transform,
        draw_entity: F,
    ) where
        R: gfx::Resources,
        F: FnMut(&gfx::Slice<R>,
              handle::Buffer<R, geometry_pass::Vertex>,
              &Self::Locals);
}

// LightTransform impls

impl LightTransform for components::DirectionalLight {
    type Transform = cgmath::Matrix4<f32>;
    type RequiredData = (cgmath::Point3<f32>, cgmath::Vector3<f32>);
    type ShaderStruct = lighting::DirectionalLight;

    fn get_light_space_transform(&self, data: Self::RequiredData) -> Self::Transform {
        let (position, direction) = data;

        let proj: cgmath::Matrix4<_> = self.projection_matrix.into();

        let view =
            cgmath::Matrix4::look_at(position, position + direction, cgmath::Vector3::unit_z());

        (proj * view)
    }
}

/// 6 view matrices, required for rendering to a cubemap
pub type ViewMatrices = [cgmath::Matrix4<f32>; 6];

/// The transform data for `PointLight`
///
/// Point light shadow mapping is more complex than shadow mapping for other light types, so this
/// struct groups the additional data required.
#[derive(Clone, Copy)]
pub struct PointLightTransform {
    pub matrices: ViewMatrices,
    pub light_pos: cgmath::Point3<f32>,
    pub far_plane: f32,
}

impl LightTransform for components::PointLight {
    type Transform = PointLightTransform;
    type RequiredData = (cgmath::Point3<f32>, AspectRatio);
    type ShaderStruct = lighting::PointLight;

    fn get_light_space_transform(&self, data: Self::RequiredData) -> Self::Transform {

        use cgmath::{Matrix4, vec3};

        let (pos, aspect_ratio) = data;

        // The projection matrix is constant for each transform matrix
        let proj = cgmath::perspective(
            cgmath::Deg(90.0),
            aspect_ratio.0,
            self.projection.near(),
            self.projection.far(),
        );

        let mut transforms =
            [
                // FIXME: Remove the sign flip on X and Z directions
                Matrix4::look_at(pos, pos + vec3(-1.0, 0.0, 0.0), vec3(0.0, 0.0, -1.0)),
                Matrix4::look_at(pos, pos + vec3(1.0, 0.0, 0.0), vec3(0.0, 0.0, -1.0)),
                Matrix4::look_at(pos, pos + vec3(0.0, 0.0, 1.0), vec3(0.0, 1.0, 0.0)),
                Matrix4::look_at(pos, pos + vec3(0.0, 0.0, -1.0), vec3(0.0, -1.0, 0.0)),
                Matrix4::look_at(pos, pos + vec3(0.0, -1.0, 0.0), vec3(0.0, 0.0, -1.0)),
                Matrix4::look_at(pos, pos + vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, -1.0)),
            ];

        for t in &mut transforms {
            *t = proj * *t;
        }

        PointLightTransform {
            matrices: transforms,
            light_pos: pos,
            far_plane: self.projection.far(),
        }
    }
}

impl LightTransform for components::SpotLight {
    type Transform = cgmath::Matrix4<f32>;
    type RequiredData = (cgmath::Point3<f32>, cgmath::Vector3<f32>, AspectRatio);
    type ShaderStruct = lighting::SpotLight;

    fn get_light_space_transform(&self, data: Self::RequiredData) -> Self::Transform {
        let (position, direction, aspect_ratio) = data;

        let outer_cutoff: cgmath::Rad<f32> = Angle::acos(self.cos_outer_cutoff().0);
        let fov = outer_cutoff * 2.0;

        let proj = cgmath::perspective(
            fov,
            aspect_ratio.0,
            self.projection.near(),
            self.projection.far(),
        );

        let view =
            cgmath::Matrix4::look_at(position, position + direction, cgmath::Vector3::unit_z());

        proj * view
    }
}

// LightShadows impls

impl LightShadows for components::DirectionalLight {
    type Locals = shadow::directional::Locals;

    fn render_shadow_map<R, F>(
        drawable: &draw::DrawableStorage<R>,
        transform: Self::Transform,
        mut draw_entity: F,
    ) where
        R: gfx::Resources,
        F: FnMut(&gfx::Slice<R>,
              handle::Buffer<R, geometry_pass::Vertex>,
              &Self::Locals),
    {
        let mut locals = shadow::directional::Locals {
            light_space_matrix: transform.into(),
            model: cgmath::Matrix4::identity().into(),
        };

        // Draw each entity to the shadow map
        for d in drawable.join() {
            // Get model matrix
            let model = d.param().get_model_matrix();
            locals.model = model.into();

            draw_entity(d.slice(), d.vertex_buffer().clone(), &locals);
        }
    }
}

impl LightShadows for components::PointLight {
    type Locals = (shadow::point::Locals, [shadow::point::ShadowMatrix; 6]);

    fn render_shadow_map<R, F>(
        drawable: &draw::DrawableStorage<R>,
        transform: Self::Transform,
        mut draw_entity: F,
    ) where
        R: gfx::Resources,
        F: FnMut(&gfx::Slice<R>,
              handle::Buffer<R, geometry_pass::Vertex>,
              &Self::Locals),
    {
        let matrices = [
            shadow::point::ShadowMatrix::from(transform.matrices[0]),
            shadow::point::ShadowMatrix::from(transform.matrices[1]),
            shadow::point::ShadowMatrix::from(transform.matrices[2]),
            shadow::point::ShadowMatrix::from(transform.matrices[3]),
            shadow::point::ShadowMatrix::from(transform.matrices[4]),
            shadow::point::ShadowMatrix::from(transform.matrices[5]),
        ];

        let mut locals = shadow::point::Locals {
            model: cgmath::Matrix4::identity().into(),
            light_pos: transform.light_pos.into(),
            far_plane: transform.far_plane,
        };

        // Draw each entity to the shadow map
        for d in drawable.join() {
            // Get model matrix
            let model = d.param().get_model_matrix();
            locals.model = model.into();

            draw_entity(d.slice(), d.vertex_buffer().clone(), &(locals, matrices));
        }
    }
}

impl LightShadows for components::SpotLight {
    type Locals = shadow::spot::Locals;

    fn render_shadow_map<R, F>(
        drawable: &draw::DrawableStorage<R>,
        transform: Self::Transform,
        mut draw_entity: F,
    ) where
        R: gfx::Resources,
        F: FnMut(&gfx::Slice<R>,
              handle::Buffer<R, geometry_pass::Vertex>,
              &Self::Locals),
    {
        let mut locals = shadow::spot::Locals {
            light_space_matrix: transform.into(),
            model: cgmath::Matrix4::identity().into(),
        };

        // Draw each entity to the shadow map
        for d in drawable.join() {
            // Get model matrix
            let model = d.param().get_model_matrix();
            locals.model = model.into();

            draw_entity(d.slice(), d.vertex_buffer().clone(), &locals);
        }
    }
}
