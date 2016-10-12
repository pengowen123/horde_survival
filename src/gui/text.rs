use glutin::Window;
use cgmath::Point2;
use collision::{Aabb, Aabb2};
use rusttype::{PositionedGlyph, point, vector};

use hsgraphics::*;
use gamestate::GameState;
use gameloop::LoopType;
use consts::text::*;
use gui::*;

pub struct Text {
    glyphs: Vec<PositionedGlyph<'static>>,
}

pub struct TextInfo {
    pub x: f32,
    pub y: f32,
    pub size: f32,
}

impl Text {
    fn new_maybe_aligned<S: AsRef<str>>(text: S, mut info: TextInfo, graphics: &mut GraphicsState, align: Option<Align>) -> Text {
        to_rusttype_coords(&mut info, graphics.window_size);

        Text {
            glyphs: graphics.layout_text(text.as_ref(), info, align),
        }
    }

    pub fn new<S: AsRef<str>>(text: S, info: TextInfo, graphics: &mut GraphicsState) -> Text {
        Text::new_maybe_aligned(text, info, graphics, None)
    }

    pub fn new_on_button<S: AsRef<str>>(text: S, rect: &Aabb2<f32>, graphics: &mut GraphicsState) -> Text {
        let (min, max) = (rect.min(), rect.max());
        let info = TextInfo::new(min.x, max.y, (max.y - min.y) * BUTTON_TEXT_HEIGHT);

        Text::new(text, info, graphics)
    }

    pub fn new_aligned<S: AsRef<str>>(text: S, size: f32, align: Align, graphics: &mut GraphicsState) -> Text {
        Text::new_maybe_aligned(text, TextInfo::new(0.0, 0.0, size), graphics, Some(align))
    }
}

impl UIObject for Text {
    fn draw(&self, graphics: &mut GraphicsState) {
        for glyph in &self.glyphs {
            graphics.cache.cache.queue_glyph(0, glyph.clone());
        }

        let encoder = &mut graphics.encoder;
        let texture = &graphics.cache.texture;

        graphics.cache.cache.cache_queued(|rect, data| {
            let offset = [rect.min.x as u16, rect.min.y as u16];
            let size = [rect.width() as u16, rect.height() as u16];

            let new_data = data.iter().map(|x| [0, 0, 0, *x]).collect::<Vec<_>>();

            update_cache_texture(encoder, texture, offset, size, &new_data);
        }).expect("Failed to cache glyphs");

        let dpi = graphics.dpi;
        let (w, h) = (graphics.window_size.0 as f32 * dpi, graphics.window_size.1 as f32 * dpi);
        let origin = point(0.0, 0.0);

        let mut vertex_data: Vec<gfx2d::Vertex> = Vec::new();
        let mut index_data: Vec<u16> = Vec::new();
        let mut i = 0;

        for g in &self.glyphs {
            let cache = &graphics.cache.cache;

            let (uv_rect, screen_rect) = match cache.rect_for(0, g) {
                Ok(Some(r)) => r,
                _ => continue,
            };

            let min = origin + (vector(screen_rect.min.x as f32 / w - 0.5, 1.0 - screen_rect.min.y as f32 / h - 0.5)) * 2.0;
            let max = origin + (vector(screen_rect.max.x as f32 / w - 0.5, 1.0 - screen_rect.max.y as f32 / h - 0.5)) * 2.0;
            let uv_min = uv_rect.min;
            let uv_max = uv_rect.max;

            let vertices = shape!(
                [min.x, max.y], [uv_min.x, uv_max.y],
                [min.x, min.y], [uv_min.x, uv_min.y],
                [max.x, min.y], [uv_max.x, uv_min.y],
                [max.x, max.y], [uv_max.x, uv_max.y]
            );

            vertex_data.extend_from_slice(&vertices);
            index_data.extend_from_slice(&[i,
                                           i + 1,
                                           i + 2,
                                           i,
                                           i + 2,
                                           i + 3]);

            i += 4;
        }

        let object = object2d::Object2d::from_slice_indices(&mut graphics.factory,
                                                            &vertex_data,
                                                            &index_data,
                                                            graphics.cache.texture_view.clone());

        object.encode(encoder, &graphics.pso2d, &mut graphics.data2d);
    }

    fn is_selected(&self, _: Point2<f32>) -> bool { false }
    fn select(&mut self,
              _: &mut Option<u32>,
              state: &UIState,
              _: &mut GameState,
              _: &mut LoopType,
              _: &Window,
              _: &mut GraphicsState) -> UIState { state.clone() }

    fn get_layer(&self) -> usize { 2 }
}

impl TextInfo {
    pub fn new(x: f32, y: f32, size: f32) -> TextInfo {
        TextInfo {
            x: x,
            y: y,
            size: size * TEXT_HEIGHT,
        }
    }
}
