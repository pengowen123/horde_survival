use conrod::{self, render};
use conrod::render::{Text, PrimitiveKind};
use conrod::text::{font, rt};

use hsgraphics::GraphicsState;
use hsgraphics::gfx2d::Vertex;
use hsgraphics::gfx_gui::{GUIObject, Vertex as GVertex};
use hsgraphics::texture::{Texture, update_cache_texture};
use hsgraphics::object2d::Object2d;
use hslog::CanUnwrap;
use gui::utils::*;

#[allow(unused_variables)]
pub fn draw_primitives<'a>(mut primitives: render::Primitives<'a>,
                           (w, h): (u32, u32),
                           graphics: &mut GraphicsState,
                           image_map: &conrod::image::Map<Texture>) {

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
            PrimitiveKind::Rectangle { color } => {
                primitive_rectangle(color, rect, (screen_width, screen_height), graphics);
            },
            PrimitiveKind::Image { color, source_rect } => {
                primitive_image(id,
                                color,
                                rect,
                                source_rect,
                                (screen_width, screen_height),
                                graphics,
                                image_map)
            },
            _ => {
                let kind_text = match kind {
                    PrimitiveKind::Rectangle { .. } => "Rectangle",
                    PrimitiveKind::Polygon { .. } => "Polygon",
                    PrimitiveKind::Lines { .. } => "Lines",
                    PrimitiveKind::Image { .. } => "Image",
                    PrimitiveKind::Text { .. } => "Text",
                    PrimitiveKind::Other(_) => continue,
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
                  screen_size: (f32, f32),
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
        min: rt_to_gl_pos(screen_rect.min, origin, screen_size),
        max: rt_to_gl_pos(screen_rect.max, origin, screen_size),
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

fn primitive_rectangle(color: conrod::Color,
                       rect: conrod::Rect, 
                       screen_size: (f32, f32),
                       graphics: &mut GraphicsState) {

    let color = color.to_fsa();
    let v = |p: conrod::Point| GVertex::new([p[0] as f32, p[1] as f32], color);

    let rect = conrod_to_gl_rect(rect, screen_size);
    let vertices = [
        v(rect.bottom_left()),
        v(rect.top_left()),
        v(rect.top_right()),
        v(rect.bottom_right()),
    ];
    let indices = [0, 1, 2, 0, 3, 2];

    let rectangle = GUIObject::new(&mut graphics.factory, &vertices, &indices);

    rectangle.encode(&mut graphics.encoder, &graphics.pso_gui, &mut graphics.data_gui);
}

fn primitive_image(id: conrod::widget::Id,
                   color: Option<conrod::Color>,
                   rect: conrod::Rect,
                   uv_rect: Option<conrod::Rect>,
                   screen_size: (f32, f32),
                   graphics: &mut GraphicsState,
                   image_map: &conrod::image::Map<Texture>) {

    let uv_rect = uv_rect.unwrap_or_else(|| conrod::Rect::from_xy_dim([0.5; 2], [1.0; 2]));
    let color = color.unwrap_or(conrod::Color::Rgba(0.0, 0.0, 0.0, 1.0)).to_fsa();
    let v = |p: conrod::Point, uv: conrod::Point| {
        Vertex::new_colored([p[0] as f32, p[1] as f32], [uv[0] as f32, uv[1] as f32], color)
    };

    let rect = conrod_to_gl_rect(rect, screen_size);
    let vertices = [
        v(rect.bottom_left(), uv_rect.top_left()),
        v(rect.top_left(), uv_rect.bottom_left()),
        v(rect.top_right(), uv_rect.bottom_right()),
        v(rect.bottom_right(), uv_rect.top_right()),
    ];
    let indices = [0, 1, 2, 0, 3, 2];

    let texture = unwrap_or_log!(image_map.get(&id), "Failed to get texture with ID {} from image_map", id.index());

    let object = Object2d::from_slice_indices(&mut graphics.factory, &vertices, &indices, texture.clone());

    object.encode(&mut graphics.encoder, &graphics.pso2d, &mut graphics.data2d);
}
