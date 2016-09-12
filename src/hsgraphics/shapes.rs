use consts::graphics::minimap::*;
use world::Direction;
use hsgraphics::utils::*;
use hsgraphics::gfx_utils::*;

macro_rules! shape {
    ($color:expr, $([$x:expr, $y:expr]),*) => {{
        [$(
            Vertex { pos: [$x + MINIMAP_LOCATION.0, $y + MINIMAP_LOCATION.1], color: $color.clone() },
         )*]
    }}
}

pub fn rotate_point(point: &mut [f32; 2], pivot: &[f32; 2], mut angle: f32) {
    angle = Direction(angle as f64).as_radians() as f32;

    let rot_y = angle.sin();
    let rot_x = angle.cos();

    point[0] -= pivot[0];
    point[1] -= pivot[1];

    let x = point[0] * rot_x - point[1] * rot_y;
    let y = point[0] * rot_y + point[1] * rot_x;

    point[0] = x + pivot[0];
    point[1] = y + pivot[1];
}

pub fn rotate_shape(shape: &mut [Vertex], pivot: [f32; 2], angle: f32) {
    // TODO: inline rotate_point here to avoid recalculation of sine and cosine
    for vertex in shape {
        rotate_point(&mut vertex.pos, &pivot, angle)
    }
}

pub fn square(position: [f32; 2], size: f32, color: Color, rotation: f32) -> [Vertex; 6] {
    let x = position[0] - size / 2.0;
    let y = position[1] - size / 2.0;
    let center = [x + size / 2.0, y + size / 2.0];
    let (scale_x, scale_y) = get_scales(size);

    let mut square = shape!(
        color,
        [x, y],
        [x + size, y],
        [x, y + size],
        [x + size, y],
        [x, y + size],
        [x + size, y + size]
    );

    rotate_shape(&mut square, center, rotation);

    for vertex in square.iter_mut() {
        vertex.pos[0] *= scale_x;
        vertex.pos[1] *= scale_y;
    }

    square
}
