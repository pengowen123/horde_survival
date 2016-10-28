use conrod::text::rt;
use conrod::{Rect, Range};

use hsgraphics::gfx2d::Vertex;

pub fn rt_to_gl_pos(rt_pos: rt::Point<i32>,
                    origin: &rt::Point<f32>,
                    (screen_width, screen_height): (f32, f32)) -> rt::Point<f32> {

    *origin + rt::vector(rt_pos.x as f32 / screen_width - 0.5,
                         1.0 - rt_pos.y as f32 / screen_height - 0.5) * 2.0
}

pub fn conrod_to_gl_rect(rect: Rect, (screen_width, screen_height): (f32, f32)) -> Rect {
    let (screen_width, screen_height) = (screen_width as f64, screen_height as f64);

    let sx = |x| x * 2.0 / screen_width;
    let sy = |y| y * 2.0 / screen_height;

    Rect {
        x: Range::new(sx(rect.x.start), sx(rect.x.end)),
        y: Range::new(sy(rect.y.start), sy(rect.y.end)),
    }
}

pub fn vertex(pos: [f32; 2], uv: [f32; 2], color: [f32; 4]) -> Vertex {
    Vertex::new_colored(pos, uv, color)
}
