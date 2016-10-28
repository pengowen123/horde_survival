use conrod::text::GlyphCache;
use gfx::handle;
use gfx_device_gl;

use hsgraphics::{Texture, SurfaceFormat};

pub struct TextCache {
    pub cache: GlyphCache,
    pub texture: handle::Texture<gfx_device_gl::Resources, SurfaceFormat>,
    pub texture_view: Texture,
}

impl TextCache {
    pub fn new(cache: GlyphCache,
               texture: handle::Texture<gfx_device_gl::Resources, SurfaceFormat>,
               texture_view: Texture) -> TextCache {

        TextCache {
            cache: cache,
            texture: texture,
            texture_view: texture_view,
        }
    }
}
