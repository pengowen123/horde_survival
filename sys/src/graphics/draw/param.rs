//! Components to pass additional parameters to the shaders

use cgmath;

use world;

/// A component to store a translation to apply to an entity's model
pub struct ShaderParamTranslation(world::components::Position);

/// A component to store a rotation to apply to an entity's model
pub struct ShaderParamRotation(cgmath::Matrix4<f32>);

// TODO: Finish these ccomponents:
//       Make fields with default values in Drawable
//       Make systems for each of these components that write to their respective fields on Drawable
