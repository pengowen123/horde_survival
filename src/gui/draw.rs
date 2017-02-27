//! Functions for drawing the primitives that form the GUI

use conrod::{self, render};
use conrod::render::{Text, PrimitiveKind};
use conrod::text::{font, rt};

use hsgraphics::GraphicsState;
use hsgraphics::{gfx2d, gfx_gui, object2d, object_gui};
use hsgraphics::texture::{Texture, update_texture};
use gui::utils::*;
use gui::crop;

/// Draws a set of conrod primitives
pub fn draw_primitives<'a>(mut primitives: render::Primitives<'a>,
                           (w, h): (u32, u32),
                           graphics: &mut GraphicsState,
                           image_map: &conrod::image::Map<Texture>) {

    // Get dimensions
    let (screen_width, screen_height) = (w as f32 * graphics.dpi, h as f32 * graphics.dpi);
    // The center of the screen
    let origin = rt::point(0.0, 0.0);

    // Loop through each primitive, and based on the primitive's kind call the appropriate function
    while let Some(render::Primitive { id, kind, scizzor, rect }) = primitives.next() {
        match kind {
            PrimitiveKind::Text { color, text, font_id } => {
                primitive_text(text,
                               color,
                               scizzor,
                               font_id,
                               &origin,
                               (screen_width, screen_height),
                               graphics);
            }
            PrimitiveKind::Rectangle { color } => {
                primitive_rectangle(color,
                                    rect,
                                    scizzor,
                                    (screen_width, screen_height),
                                    graphics);
            }
            PrimitiveKind::Image { color, source_rect } => {
                primitive_image(id,
                                color,
                                rect,
                                source_rect,
                                scizzor,
                                (screen_width, screen_height),
                                graphics,
                                image_map)
            }
            // The GUI doesn't contain any other kinds of primitives, so their draw functions aren't
            // implemented
            // NOTE: If any unimplemented primitives are used, a warning will appear in the logs
            _ => {
                let kind_text = match kind {
                    PrimitiveKind::Rectangle { .. } => "Rectangle",
                    PrimitiveKind::Polygon { .. } => "Polygon",
                    // NOTE: If lines are ever needed, see the glium backend for conrod for line
                    //       drawing
                    PrimitiveKind::Lines { .. } => "Lines",
                    PrimitiveKind::Image { .. } => "Image",
                    PrimitiveKind::Text { .. } => "Text",
                    PrimitiveKind::Other(_) => continue,
                };
                warn!("Drawing unknown primitive: {:?}", kind_text);
            }
        }
    }
}

/// Draws text
fn primitive_text(text: Text,
                  color: conrod::Color,
                  scizzor: conrod::Rect,
                  font_id: font::Id,
                  origin: &rt::Point<f32>,
                  screen_size: (f32, f32),
                  graphics: &mut GraphicsState) {

    // Queue each glyph to be cached
    let positioned_glyphs = text.positioned_glyphs(graphics.dpi);
    for glyph in positioned_glyphs {
        graphics.cache.cache.queue_glyph(font_id.index(), glyph.clone());
    }

    let encoder = &mut graphics.encoder;
    let cache = &mut graphics.cache.cache;
    let cache_tex = &graphics.cache.texture;

    // Cache each glyph
    cache.cache_queued(|rect, data| {
            let offset = [rect.min.x as u16, rect.min.y as u16];
            let size = [rect.width() as u16, rect.height() as u16];

            let new_data = data.iter().map(|x| [0, 0, 0, *x]).collect::<Vec<_>>();

            update_texture(encoder, cache_tex, offset, size, &new_data);
        })
        .unwrap();

    let color = color.to_fsa();
    let cache_id = font_id.index();

    // Converts RustType rects to OpenGL rects
    let to_gl_rect = |screen_rect: rt::Rect<i32>| {
        rt::Rect {
            min: rt_to_gl_pos(screen_rect.min, origin, screen_size),
            max: rt_to_gl_pos(screen_rect.max, origin, screen_size),
        }
    };

    // Calculate the vertices of the text, including texture coordinates
    // NOTE: This might be inefficient (because of flatmap, maybe chain also), do some profiling to
    //       maybe optimize it
    let vertices = positioned_glyphs.into_iter()
        // Get the coordinates of the rectangle of the glyph
        // NOTE: cache.rect_for() returns Result<Option<T>, E>
        //       The unwrap_or(None) is not useless because of this
        .filter_map(|g| cache.rect_for(cache_id, g).ok().unwrap_or(None))
        .map(|(uv_rect, screen_rect)| {
            let screen_rect = to_gl_rect(screen_rect);

            crop::text(uv_rect, screen_rect, scizzor)
        })
        .flat_map(|(uv_rect, gl_rect)| {
            use std::iter::once;

            // Creates an iterator out of a single vertex for chaining
            let v = |pos, uv| once(vertex(pos, uv, color));

            // An iterator of the 6 vertices of the glyph
            v([gl_rect.min.x, gl_rect.max.y],
              [uv_rect.min.x, uv_rect.max.y])
                .chain(v([gl_rect.min.x, gl_rect.min.y],
                         [uv_rect.min.x, uv_rect.min.y]))
                .chain(v([gl_rect.max.x, gl_rect.min.y],
                         [uv_rect.max.x, uv_rect.min.y]))
                .chain(v([gl_rect.max.x, gl_rect.min.y],
                         [uv_rect.max.x, uv_rect.min.y]))
                .chain(v([gl_rect.max.x, gl_rect.max.y],
                         [uv_rect.max.x, uv_rect.max.y]))
                .chain(v([gl_rect.min.x, gl_rect.max.y],
                         [uv_rect.min.x, uv_rect.max.y]))
        })
        .collect::<Vec<_>>();

    // Create an object from the vertices and draw it
    let text = object2d::Object2d::new(&mut graphics.factory,
                                       &vertices,
                                       graphics.cache.texture_view.clone(),
                                       ());
    text.encode(encoder, &graphics.pso2d, &mut graphics.data2d);
}

/// Draws a rectangle
fn primitive_rectangle(color: conrod::Color,
                       rect: conrod::Rect,
                       scizzor: conrod::Rect,
                       screen_size: (f32, f32),
                       graphics: &mut GraphicsState) {

    let color = color.to_fsa();
    // Creates a GVertex from a point
    let v = |p: conrod::Point| gfx_gui::Vertex::new([p[0] as f32, p[1] as f32], color);

    // Crop the rectangle
    let rect = crop::rect(rect, scizzor);

    let rect = conrod_to_gl_rect(rect, screen_size);
    // Create the vertices of the rectangle
    let vertices = [rect.bottom_left(), rect.top_left(), rect.top_right(), rect.bottom_right()]
        .into_iter()
        .map(|r| v(*r))
        .collect::<Vec<_>>();
    let indices = [0, 1, 2, 0, 3, 2];

    // Create an object from the vertices and draw it
    let rectangle = object_gui::ObjectGUI::new(&mut graphics.factory, &vertices, &indices);
    rectangle.encode(&mut graphics.encoder,
                     &graphics.pso_gui,
                     &mut graphics.data_gui);
}

/// Draws an image
#[allow(too_many_arguments)]
fn primitive_image(id: conrod::widget::Id,
                   color: Option<conrod::Color>,
                   rect: conrod::Rect,
                   uv_rect: Option<conrod::Rect>,
                   scizzor: conrod::Rect,
                   screen_size: (f32, f32),
                   graphics: &mut GraphicsState,
                   image_map: &conrod::image::Map<Texture>) {

    // If no texture coordinates are provided, use the entire image
    let uv_rect = uv_rect.unwrap_or_else(|| conrod::Rect::from_xy_dim([0.5; 2], [1.0; 2]));
    // If no color is provided, use no color (leave the image unchanged)
    let color = color.unwrap_or(conrod::Color::Rgba(0.0, 0.0, 0.0, 1.0)).to_fsa();
    // Creates a colored vertex
    let v = |p: conrod::Point, uv: conrod::Point| {
        gfx2d::Vertex::new_colored([p[0] as f32, p[1] as f32],
                                   [uv[0] as f32, uv[1] as f32],
                                   color)
    };

    // TODO: Uncomment when image cropping works
    // Crop the image
    let (rect, uv_rect) = crop::image(rect, uv_rect, scizzor);

    let rect = conrod_to_gl_rect(rect, screen_size);
    // Create the vertices of the image
    let vertices = [v(rect.bottom_left(), uv_rect.top_left()),
                    v(rect.top_left(), uv_rect.bottom_left()),
                    v(rect.top_right(), uv_rect.bottom_right()),
                    v(rect.bottom_right(), uv_rect.top_right())];
    let indices = [0u32, 1, 2, 0, 3, 2];

    // Get the image
    let texture = image_map.get(&id).unwrap_or_else(|| {
        crash!("Failed to get texture with ID {} from image_map",
               id.index())
    });
    // Create the object from the vertices and draw it
    let object = object2d::Object2d::new(&mut graphics.factory,
                                         &vertices,
                                         texture.clone(),
                                         &indices[..]);
    object.encode(&mut graphics.encoder, &graphics.pso2d, &mut graphics.data2d);
}
