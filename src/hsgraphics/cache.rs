use rusttype::gpu_cache::Cache;
use gfx::handle;
use gfx_device_gl;

use hsgraphics::{Texture, SurfaceFormat};

pub struct GlyphCache {
    pub cache: Cache,
    pub texture: handle::Texture<gfx_device_gl::Resources, SurfaceFormat>,
    pub texture_view: Texture,
}

impl GlyphCache {
    pub fn new(cache: Cache, texture: handle::Texture<gfx_device_gl::Resources, SurfaceFormat>, texture_view: Texture) -> GlyphCache {
        GlyphCache {
            cache: cache,
            texture: texture,
            texture_view: texture_view,
        }
    }
}
