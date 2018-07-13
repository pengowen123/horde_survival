//! Asset management

pub mod obj;
pub(crate) mod shader;
mod utils;

pub use self::utils::read_bytes;

/// Returns the path to the shader with the provided name
pub fn get_shader_path(name: &str) -> String {
    format!(
        "{}/../test_assets/shaders/{}.glsl",
        env!("CARGO_MANIFEST_DIR"),
        name
    )
}

/// Returns the path to the model file given the name of a model and a suffix to attach to it
pub fn get_model_file_path(name: &str, suffix: &str) -> String {
    format!(
        "{}/../test_assets/models/{}{}",
        env!("CARGO_MANIFEST_DIR"),
        name,
        suffix
    )
}
