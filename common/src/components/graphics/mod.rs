//! Common components for graphics
//!
//! Mainly here so the `rendergraph` crate can access them.

use cgmath::{self, One};
use gfx::{self, handle};
use specs;
use skeletal_animation::{controller, Skeleton, Transform};
use skeletal_animation::dual_quaternion::DualQuaternion;

mod particles;

pub use self::particles::*;

/// The maximum number of joints per object
pub const MAX_JOINTS: u16 = 64;

/// A view into a texture
pub type TextureView<R> = handle::ShaderResourceView<R, [f32; 4]>;

/// The skinning transform type used by `DrawableSkeletal`
pub type SkinningTransform = DualQuaternion<f32>;

/// An animation controller
pub type AnimationController = controller::AnimationController<SkinningTransform>;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
        normal: [f32; 3] = "a_Normal",
        uv: [f32; 2] = "a_Uv",
    }

    vertex VertexSkeletal {
        pos: [f32; 3] = "a_Pos",
        normal: [f32; 3] = "a_Normal",
        uv: [f32; 2] = "a_Uv",
        joints: [i32; 4] = "a_JointIndices",
        joint_weights: [f32; 4] = "a_JointWeights",
    }

    constant Material {
        shininess: f32 = "u_Material_shininess",
    }
}

impl Vertex {
    pub fn new(pos: [f32; 3], uv: [f32; 2], normal: [f32; 3]) -> Self {
        Self { pos, normal, uv }
    }
}

impl VertexSkeletal {
    pub fn new(
        pos: [f32; 3],
        uv: [f32; 2],
        normal: [f32; 3],
        joints: [i32; 4],
        joint_weights: [f32; 4],
    ) -> Self {
        Self {
            pos,
            normal,
            uv,
            joints,
            joint_weights,
        }
    }
}

impl Material {
    pub fn new(shininess: f32) -> Self {
        Self { shininess }
    }
}

#[derive(Clone)]
struct DrawableInner<R: gfx::Resources> {
    diffuse: TextureView<R>,
    specular: TextureView<R>,
    slice: gfx::Slice<R>,
    material: Material,
    param: ShaderParam,
}

impl<R: gfx::Resources> DrawableInner<R> {
    pub fn new(
        slice: gfx::Slice<R>,
        diffuse: TextureView<R>,
        specular: TextureView<R>,
        material: Material,
    ) -> Self {
        DrawableInner {
            slice,
            diffuse,
            specular,
            material,
            param: Default::default(),
        }
    }

    fn diffuse(&self) -> &TextureView<R> {
        &self.diffuse
    }

    fn specular(&self) -> &TextureView<R> {
        &self.specular
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn slice(&self) -> &gfx::Slice<R> {
        &self.slice
    }

    fn param(&self) -> &ShaderParam {
        &self.param
    }

    fn set_shader_param(&mut self, param: ShaderParam) {
        self.param = param;
    }
}

/// A component that stores the information needed to draw an entity
#[derive(Clone)]
pub struct Drawable<R: gfx::Resources> {
    vertex_buffer: handle::Buffer<R, Vertex>,
    inner: DrawableInner<R>,
}

impl<R: gfx::Resources> Drawable<R> {
    pub fn new(
        vertex_buffer: handle::Buffer<R, Vertex>,
        slice: gfx::Slice<R>,
        diffuse: TextureView<R>,
        specular: TextureView<R>,
        material: Material,
    ) -> Self {
        Self {
            vertex_buffer,
            inner: DrawableInner::new(slice, diffuse, specular, material),
        }
    }

    /// Returns a reference to the vertex buffer
    pub fn vertex_buffer(&self) -> &handle::Buffer<R, Vertex> {
        &self.vertex_buffer
    }

    /// Returns a reference to the diffuse map
    pub fn diffuse(&self) -> &TextureView<R> {
        self.inner.diffuse()
    }

    /// Returns a reference to the specular map
    pub fn specular(&self) -> &TextureView<R> {
        self.inner.specular()
    }

    /// Returns a reference to the material
    pub fn material(&self) -> &Material {
        self.inner.material()
    }

    /// Returns a reference to the vertex buffer slice
    pub fn slice(&self) -> &gfx::Slice<R> {
        self.inner.slice()
    }

    /// Returns a reference to the shader parameters
    pub fn param(&self) -> &ShaderParam {
        self.inner.param()
    }

    /// Sets the shader parameters to the provided value
    pub fn set_shader_param(&mut self, param: ShaderParam) {
        self.inner.set_shader_param(param)
    }
}

pub struct DrawableSkeletal<R: gfx::Resources> {
    vertex_buffer: handle::Buffer<R, VertexSkeletal>,
    skeleton: Skeleton,
    animation_controller: AnimationController,
    skinning_transforms: Vec<SkinningTransform>,
    inner: DrawableInner<R>,
}

impl<R: gfx::Resources> DrawableSkeletal<R> {
    pub fn new(
        animation_controller: AnimationController,
        skeleton: Skeleton,
        vertex_buffer: handle::Buffer<R, VertexSkeletal>,
        slice: gfx::Slice<R>,
        diffuse: TextureView<R>,
        specular: TextureView<R>,
        material: Material,
    ) -> Self {
        Self {
            vertex_buffer,
            animation_controller,
            skeleton,
            skinning_transforms: Vec::new(),
            inner: DrawableInner::new(slice, diffuse, specular, material),
        }
    }

    /// Returns a reference to the vertex buffer
    pub fn vertex_buffer(&self) -> &handle::Buffer<R, VertexSkeletal> {
        &self.vertex_buffer
    }

    /// Returns a reference to the diffuse map
    pub fn diffuse(&self) -> &TextureView<R> {
        self.inner.diffuse()
    }

    /// Returns a reference to the specular map
    pub fn specular(&self) -> &TextureView<R> {
        self.inner.specular()
    }

    /// Returns a reference to the material
    pub fn material(&self) -> &Material {
        self.inner.material()
    }

    /// Returns a reference to the vertex buffer slice
    pub fn slice(&self) -> &gfx::Slice<R> {
        self.inner.slice()
    }

    /// Returns a reference to the shader parameters
    pub fn param(&self) -> &ShaderParam {
        self.inner.param()
    }

    /// Sets the shader parameters to the provided value
    pub fn set_shader_param(&mut self, param: ShaderParam) {
        self.inner.set_shader_param(param)
    }

    /// Returns a reference to the skeleton
    pub fn skeleton(&self) -> &Skeleton {
        &self.skeleton
    }

    /// Returns a reference to the animation controller
    pub fn animation_controller(&self) -> &AnimationController {
        &self.animation_controller
    }

    /// Updates the state of the animation controller and calculates new skinning transforms
    pub fn update_animation_controller(&mut self, exit_dt: f64) {
        let mut global_poses = [SkinningTransform::identity(); MAX_JOINTS as usize];

        self.animation_controller
            .get_output_pose(exit_dt, &mut global_poses[0..self.skeleton.joints.len()]);

        self.skinning_transforms = self.skeleton.joints.iter().enumerate().map(|(i, joint)| {
            global_poses[i].concat(DualQuaternion::from_matrix(joint.inverse_bind_pose))
        }).collect();
    }

    /// Returns the current skinning transforms
    pub fn skinning_transforms(&self) -> &[SkinningTransform] {
        &self.skinning_transforms
    }
}

/// A 3D rotation
pub type Rotation = cgmath::Matrix4<f32>;

/// A 3D scale
pub type Scale = cgmath::Matrix4<f32>;

/// A 3D translation
pub type Translation = cgmath::Matrix4<f32>;

/// A type that stores all individual parameters to pass to the graphics system
#[derive(Clone, Copy, Debug)]
pub struct ShaderParam {
    translation: Translation,
    rotation: Rotation,
    scale: Scale,
}

impl ShaderParam {
    pub fn new(translation: Translation, rotation: Rotation, scale: Scale) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn set_translation(&mut self, new_translation: Translation) {
        self.translation = new_translation;
    }

    pub fn set_rotation(&mut self, new_rotation: Rotation) {
        self.rotation = new_rotation;
    }

    pub fn set_scale(&mut self, new_scale: Scale) {
        self.scale = new_scale;
    }

    /// Returns the model matrix, created from the stored translation, rotation, and scale matrices
    pub fn get_model_matrix(&self) -> cgmath::Matrix4<f32> {
        self.translation * self.rotation * self.scale
    }
}

impl Default for ShaderParam {
    fn default() -> Self {
        // Identity transformations (zero translation, zero rotation)
        ShaderParam::new(One::one(), One::one(), One::one())
    }
}

impl specs::Component for ShaderParam {
    type Storage = specs::DenseVecStorage<Self>;
}

impl<R: gfx::Resources> specs::Component for Drawable<R> {
    type Storage = specs::DenseVecStorage<Self>;
}

impl<R: gfx::Resources> specs::Component for DrawableSkeletal<R> {
    type Storage = specs::DenseVecStorage<Self>;
}
