use consts::graphics::minimap::*;
use hsgraphics::gfx2d::{self, Color};
use world::Direction;

macro_rules! shape {
    ($color:expr, $([$x:expr, $y:expr]),*) => {{
        [$(
            gfx2d::Vertex { pos: [$x + MINIMAP_LOCATION.0, $y + MINIMAP_LOCATION.1], color: $color.clone() },
         )*]
    }}
}

pub fn rotate_point(point: &mut [f32; 2], pivot: &[f32; 2], rot_x: f32, rot_y: f32) {
    point[0] -= pivot[0];
    point[1] -= pivot[1];

    let x = point[0] * rot_x - point[1] * rot_y;
    let y = point[0] * rot_y + point[1] * rot_x;

    point[0] = x + pivot[0];
    point[1] = y + pivot[1];
}

pub fn rotate_shape(shape: &mut [gfx2d::Vertex], pivot: [f32; 2], angle: f32) {
    let angle = Direction(angle as f64).as_radians() as f32;
    let rot_y = angle.sin();
    let rot_x = angle.cos();

    for vertex in shape {
        rotate_point(&mut vertex.pos, &pivot, rot_x, rot_y)
    }
}

pub fn square(position: [f32; 2], size: f32, color: Color, rotation: f32, scales: (f32, f32)) -> [gfx2d::Vertex; 6] {
    let zero = 0.0 - size / 2.0;
    let center = [0.0, 0.0];
    let (scale_x, scale_y) = scales;

    let mut square = shape!(
        color,
        [zero, zero],
        [zero + size, zero],
        [zero, zero + size],
        [zero + size, zero],
        [zero, zero + size],
        [zero + size, zero + size]
    );

    rotate_shape(&mut square, center, rotation);

    for vertex in square.iter_mut() {
        let pos = &mut vertex.pos;
        pos[0] += position[0];
        pos[0] *= scale_x;

        pos[1] += position[1];
        pos[1] *= scale_y;
    }

    square
}
