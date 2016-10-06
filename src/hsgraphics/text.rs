use rusttype::*;

use consts::text::TEXT_HEIGHT;
use hsgraphics::*;

impl GraphicsState {
    pub fn create_text_texture(&mut self, text: &str, height: f32) -> Texture {
        let font = self.assets.font.get().unwrap();
        let (size, texture) = text_to_texture(font, height, text);

        texture::load_texture_raw(&mut self.factory, size, &texture)
    }
}

fn calc_text_width(glyphs: &[PositionedGlyph]) -> f32 {
    glyphs.last().unwrap().pixel_bounding_box().expect("No pixel bounding box found").max.x as f32
}

// NOTE: Taken from Zone of Control
fn text_to_texture(font: &Font, height: f32, text: &str) -> ([u32; 2], Vec<u8>) {
    let scale = Scale { x: height, y: height * TEXT_HEIGHT };
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);
    let glyphs: Vec<_> = font.layout(text, scale, offset).collect();
    let pixel_height = height.ceil() as usize;
    let width = calc_text_width(&glyphs) as usize;
    let mut pixel_data = transparent(width * pixel_height * 4);
    let mapping_scale = 255.0;
    for g in glyphs {
        let bb = match g.pixel_bounding_box() {
            Some(bb) => bb,
            None => continue,
        };
        g.draw(|x, y, v| {
            let v = (v * mapping_scale + 0.5) as u8;
            let x = x as i32 + bb.min.x;
            let y = y as i32 + bb.min.y;
            let i = (x as usize + y as usize * width) * 4;
            // There's still a possibility that the glyph clips the boundaries of the bitmap
            if v > 0 && x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
                pixel_data[i] = 0;
                pixel_data[i + 1] = 0;
                pixel_data[i + 2] = 0;
                pixel_data[i + 3] = v;
            }
        });
    }

    let size = [width as u32, pixel_height as u32];
    (size, pixel_data)
}

fn transparent(size: usize) -> Vec<u8> {
    [255, 255, 255, 0].iter().cloned().cycle().take(size).collect()
}
