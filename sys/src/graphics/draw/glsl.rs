//! Types and functions for working with their GLSL counterparts

/// A GLSL `vec2`
pub type Vec2 = [f32; 2];
/// A GLSL `vec3`
pub type Vec3 = [f32; 3];
/// A GLSL `vec4`
pub type Vec4 = [f32; 4];
/// A GLSL `mat4`
pub type Mat4 = [Vec4; 4];

/// Converts a 3-component vector to a 4-component vector by appending `elem`
pub fn vec4(vec3: [f32; 3], elem: f32) -> [f32; 4] {
    [vec3[0], vec3[1], vec3[2], elem]
}
