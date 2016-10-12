use unicode_normalization::UnicodeNormalization;
use rusttype::*;
use collision::Aabb;

use hsgraphics::*;
use gui::{TextInfo, Align, rect};

impl GraphicsState {
    pub fn layout_text(&mut self, text: &str, mut info: TextInfo, align: Option<Align>) -> Vec<PositionedGlyph<'static>> {
        let font = self.assets.font.get().unwrap();
        let scale = Scale::uniform(self.dpi * info.size);
        let v_metrics = font.v_metrics(scale);
        let mut last_glyph_id = None;

        let glyphs: Vec<_> = text.nfc().filter_map(|c| font.glyph(c).map(|g| g.scaled(scale))).collect();

        if let Some(a) = align {
            let width = self.get_text_width(font, &scale, &glyphs);

            let mut rect = rect((0.0, 0.0), (width / self.window_size.0 as f32,
                                             info.size / self.window_size.1 as f32));

            a.apply(&mut rect);

            let min = rect.min();

            info.x = min.x;
            info.y = min.y;

            to_rusttype_coords(&mut info, self.window_size);
        }

        let mut caret = point(info.x, info.y + v_metrics.ascent);

        glyphs.into_iter().map(|g| {
            let glyph_id = g.id();

            if let Some(last_id) = last_glyph_id {
                caret.x += font.pair_kerning(scale, last_id, glyph_id);
            }

            let glyph = g.positioned(caret);
            caret.x += glyph.unpositioned().h_metrics().advance_width;
            last_glyph_id = Some(glyph_id);

            glyph.standalone()
        }).collect()
    }

    pub fn get_text_width(&self, font: &Font, scale: &Scale, glyphs: &[ScaledGlyph]) -> f32 {
        // FIXME: This is very inaccurate
        let mut last_glyph_id = None;

        glyphs.iter().fold(0.0, |acc, g| {
            let width = g.exact_bounding_box().map(|b| b.width()).unwrap_or(0.0);
            let kerning = last_glyph_id.map(|id| font.pair_kerning(*scale, id, g.id())).unwrap_or(0.0);
            last_glyph_id = Some(g.id());
            acc + width + kerning
        })
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
