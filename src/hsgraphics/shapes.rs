use consts::graphics::minimap::*;
use hsgraphics::utils::*;
use hsgraphics::gfx_utils::*;

macro_rules! shape {
    ($color:expr, $([$x:expr, $y:expr]),*) => {{
        [$(
            Vertex { pos: [$x + MINIMAP_LOCATION.0, $y + MINIMAP_LOCATION.1], color: $color.clone() },
         )*]
    }}
}

// TODO: Make this function and use it to make the minimap entities face their direction
pub fn rotate_shape(shape: &mut [Vertex], point: [f32; 2]) {
}

pub fn square(position: [f32; 2], size: f32, color: Color) -> [Vertex; 6] {
    let (size_x, size_y) = normalize_xy(size);
    let x = position[0];
    let y = position[1];

    shape!(
        color,
        [x, y],
        [x + size_x, y],
        [x, y + size_y],
        [x + size_x, y],
        [x, y + size_y],
        [x + size_x, y + size_y]
    )
}
