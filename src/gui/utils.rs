use conrod::text::rt;

use hsgraphics::gfx2d::Vertex;

pub fn to_gl_pos(rt_pos: rt::Point<i32>,
             origin: &rt::Point<f32>,
             (screen_width, screen_height): (f32, f32)) -> rt::Point<f32> {

    *origin + rt::vector(rt_pos.x as f32 / screen_width - 0.5,
                               1.0 - rt_pos.y as f32 / screen_height - 0.5) * 2.0
}

pub fn vertex(pos: [f32; 2], uv: [f32; 2], color: [f32; 4]) -> Vertex {
    Vertex::new_colored(pos, uv, color)
}
