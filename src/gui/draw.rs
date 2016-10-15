use conrod::{self, render};
use conrod::render::{Text, PrimitiveKind};
use conrod::text::{font, rt};

use hsgraphics::GraphicsState;
use hsgraphics::gfx2d::Vertex;
use hsgraphics::texture::update_cache_texture;
use hsgraphics::object2d::Object2d;
use gui::utils::*;

pub fn draw_primitives<'a>(mut primitives: render::Primitives<'a>,
                           (w, h): (u32, u32),
                           graphics: &mut GraphicsState) {

    let (screen_width, screen_height) = (w as f32 * graphics.dpi, h as f32 * graphics.dpi);
    let origin = rt::point(0.0, 0.0);

    while let Some(render::Primitive { id, kind, scizzor, rect }) = primitives.next() {
        match kind {
            PrimitiveKind::Text { color, text, font_id } => {
                primitive_text(text,
                               color,
                               font_id,
                               &origin,
                               (screen_width, screen_height),
                               graphics);
            },
            //PrimitiveKind::Rectangle => {
                //rect_vertices.extend();
            //},
            _ => {
                let kind_text = match kind {
                    PrimitiveKind::Rectangle { .. } => "Rectangle",
                    PrimitiveKind::Polygon { .. } => "Polygon",
                    PrimitiveKind::Lines { .. } => "Lines",
                    PrimitiveKind::Image { .. } => "Image",
                    PrimitiveKind::Text { .. } => "Text",
                    PrimitiveKind::Other(_) => "Other",
                };
                warn!("Drawing unknown primitive: {:?}", kind_text);
            },
        }
    }
}

fn primitive_text(text: Text,
                  color: conrod::Color,
                  font_id: font::Id,
                  origin: &rt::Point<f32>,
                  (screen_width, screen_height): (f32, f32),
                  graphics: &mut GraphicsState) {

    let positioned_glyphs = text.positioned_glyphs(graphics.dpi);

    for glyph in positioned_glyphs {
        graphics.cache.cache.queue_glyph(font_id.index(), glyph.clone());
    }

    let encoder = &mut graphics.encoder;
    let cache = &mut graphics.cache.cache;
    let cache_tex = &graphics.cache.texture;

    cache.cache_queued(|rect, data| {
        let offset = [rect.min.x as u16, rect.min.y as u16];
        let size = [rect.width() as u16, rect.height() as u16];

        let new_data = data.iter().map(|x| [0, 0, 0, *x]).collect::<Vec<_>>();

        update_cache_texture(encoder, cache_tex, offset, size, &new_data);
    }).unwrap();

    let color = color.to_fsa();
    let cache_id = font_id.index();

    let to_gl_rect = |screen_rect: rt::Rect<i32>| rt::Rect {
        min: to_gl_pos(screen_rect.min, origin, (screen_width, screen_height)),
        max: to_gl_pos(screen_rect.max, origin, (screen_width, screen_height)),
    };

    let vertices: Vec<_> = positioned_glyphs.into_iter()
        .filter_map(|g| cache.rect_for(cache_id, g).ok().unwrap_or(None))
        .flat_map(|(uv_rect, screen_rect)| {
            use std::iter::once;

            let gl_rect = to_gl_rect(screen_rect);
            let v = |pos, uv| once(vertex(pos, uv, color));

            v([gl_rect.min.x, gl_rect.max.y], [uv_rect.min.x, uv_rect.max.y])
                .chain(v([gl_rect.min.x, gl_rect.min.y], [uv_rect.min.x, uv_rect.min.y]))
                .chain(v([gl_rect.max.x, gl_rect.min.y], [uv_rect.max.x, uv_rect.min.y]))
                .chain(v([gl_rect.max.x, gl_rect.min.y], [uv_rect.max.x, uv_rect.min.y]))
                .chain(v([gl_rect.max.x, gl_rect.max.y], [uv_rect.max.x, uv_rect.max.y]))
                .chain(v([gl_rect.min.x, gl_rect.max.y], [uv_rect.min.x, uv_rect.max.y]))
        }).collect();

    let text = Object2d::from_slice(&mut graphics.factory, &vertices, graphics.cache.texture_view.clone());
    text.encode(encoder, &graphics.pso2d, &mut graphics.data2d);
}
