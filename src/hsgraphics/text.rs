use unicode_normalization::UnicodeNormalization;
use rusttype::*;
use collision::Aabb;

use hsgraphics::*;
use consts::text::TEXT_HEIGHT;
use gui::{TextInfo, Align, rect};

impl GraphicsState {
    pub fn layout_text(&mut self, text: &str, mut info: TextInfo, align: Option<Align>) -> Vec<PositionedGlyph<'static>> {
        let font = self.assets.font.get().unwrap();
        let scale = Scale::uniform(self.dpi * info.size);
        let v_metrics = font.v_metrics(scale);
        let mut last_glyph_id = None;

        let (glyphs, mut text_rect) = {
            let mut glyphs = Vec::new();
            let mut width = 0.0;
            let mut height = 0.0;

            for c in text.nfc() {
                let glyph = match font.glyph(c) {
                    Some(g) => g,
                    None => continue,
                };

                let scaled = glyph.scaled(scale);

                // TODO: Fix text width calculations, ask for help on reddit
                let h_metrics = scaled.h_metrics();
                width += h_metrics.left_side_bearing + h_metrics.advance_width;

                if let Some(b) = scaled.exact_bounding_box() {
                    let box_height = b.height();

                    if box_height > height {
                        height = box_height;
                    }
                }

                glyphs.push(scaled.into_unscaled());
            }

            let rect = rect((0.0, 0.0), (width / self.window_size.0 as f32,
                                         height / self.window_size.1 as f32));

            (glyphs, rect)
        };
        
        if let Some(a) = align {
            a.apply(&mut text_rect);

            //println!("{:?}", text_rect.dim());

            let caret = text_rect.min();

            info.x = caret.x;
            info.y = caret.y;

            to_rusttype_coords(&mut info, self.window_size);
        }

        let mut caret = point(info.x, info.y + v_metrics.ascent);

        let mut result = Vec::new();

        for glyph in glyphs {
            let base_id = glyph.id();

            if let Some(id) = last_glyph_id.take() {
                caret.x += font.pair_kerning(scale, id, base_id);
            }

            let glyph = glyph.scaled(scale).positioned(caret);

            caret.x += glyph.unpositioned().h_metrics().advance_width;

            result.push(glyph.standalone());

            last_glyph_id = Some(base_id);
        }

        result
    }
}

pub fn to_rusttype_coords(info: &mut TextInfo, window_size: (u32, u32)) {
        // Map window coordinates to rusttype coordinates
        info.x = (info.x + 1.0) / 2.0;
        info.y = (info.y - 1.0) / 2.0;
        info.y = -info.y;

        // Convert rusttype coordinates to pixel coordinates
        info.x *= window_size.0 as f32;
        info.y *= window_size.1 as f32;
        info.size *= window_size.1 as f32;
}
