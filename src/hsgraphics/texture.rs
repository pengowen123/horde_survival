use gfx::handle::ShaderResourceView;
use gfx_device_gl::Resources;

pub type Texture = ShaderResourceView<Resources, [f32; 4]>;

pub use image_utils::{load_texture, load_texture_raw};
