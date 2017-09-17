//! Pipeline declarations for each shadow type

pub mod directional;
pub mod point;
pub mod spot;
pub mod traits;

use graphics::draw::glsl::Mat4;

gfx_defines! {
    constant Locals {
        light_space_matrix: Mat4 = "lightSpaceMatrix",
        model: Mat4 = "model",
    }
}
