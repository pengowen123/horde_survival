//! 3d shapes

use consts::scale::WORLD_SCALE;
use hsgraphics::gfx3d::Vertex;

/// Returns a cube, given its position and the length of a side
// TODO: Add rotation
//       Not very important, because real models will be added eventually
//       Regardless, cgmath will probably help with the rotations
pub fn cube(mut position: [f32; 3], size: f32) -> ([Vertex; 24], [u16; 36]) {
    // Scale the position by world scale
    position[0] *= WORLD_SCALE;
    position[1] *= WORLD_SCALE;
    position[2] *= WORLD_SCALE;

    // Coordinates of a cube centered at the origin, with a side length of 2
    let mut v = [[1.0, 1.0, 1.0],
                 [1.0, 1.0, -1.0],
                 [-1.0, 1.0, 1.0],
                 [-1.0, 1.0, -1.0],
                 [1.0, -1.0, 1.0],
                 [-1.0, -1.0, 1.0],
                 [1.0, -1.0, -1.0],
                 [-1.0, -1.0, -1.0]];

    // Scale the cube by world scale and its size, and add its position to it
    for pos in &mut v {
        for (i, c) in pos[0..3].iter_mut().enumerate() {
            *c = position[i] + *c * WORLD_SCALE * size;
        }
    }

    // NOTE: Add rotation code before this to operate on a smaller array
    // Create the vertices
    let vertex_data = [// top
                       Vertex::new(v[5], [0.0, 0.0]),
                       Vertex::new(v[4], [1.0, 0.0]),
                       Vertex::new(v[0], [1.0, 1.0]),
                       Vertex::new(v[2], [0.0, 1.0]),
                       // bottom
                       Vertex::new(v[3], [1.0, 0.0]),
                       Vertex::new(v[1], [0.0, 0.0]),
                       Vertex::new(v[6], [0.0, 1.0]),
                       Vertex::new(v[7], [1.0, 1.0]),
                       // right
                       Vertex::new(v[6], [0.0, 0.0]),
                       Vertex::new(v[1], [1.0, 0.0]),
                       Vertex::new(v[0], [1.0, 1.0]),
                       Vertex::new(v[4], [0.0, 1.0]),
                       // left
                       Vertex::new(v[5], [1.0, 0.0]),
                       Vertex::new(v[2], [0.0, 0.0]),
                       Vertex::new(v[3], [0.0, 1.0]),
                       Vertex::new(v[7], [1.0, 1.0]),
                       // front
                       Vertex::new(v[1], [1.0, 0.0]),
                       Vertex::new(v[3], [0.0, 0.0]),
                       Vertex::new(v[2], [0.0, 1.0]),
                       Vertex::new(v[0], [1.0, 1.0]),
                       // back
                       Vertex::new(v[4], [0.0, 0.0]),
                       Vertex::new(v[5], [1.0, 0.0]),
                       Vertex::new(v[7], [1.0, 1.0]),
                       Vertex::new(v[6], [0.0, 1.0])];

    // Create the indices
    // rustfmt did this
    let index_data = [// Top face
                      0,
                      1,
                      2,
                      2,
                      3,
                      0,
                      // Bottom face
                      4,
                      5,
                      6,
                      6,
                      7,
                      4,
                      // Right face
                      8,
                      9,
                      10,
                      10,
                      11,
                      8,
                      // Left face
                      12,
                      13,
                      14,
                      14,
                      15,
                      12,
                      // Front face
                      16,
                      17,
                      18,
                      18,
                      19,
                      16,
                      // Back face
                      20,
                      21,
                      22,
                      22,
                      23,
                      20];

    (vertex_data, index_data)
}

/// Returns a plane, given its height and the length of a side
pub fn plane(height: f32, size: f32) -> ([Vertex; 4], [u16; 6]) {
    // Create the vertices
    let mut vertex_data = [Vertex::new([-0.5, -0.5, height], [0.0, 0.0]),
                           Vertex::new([0.5, -0.5, height], [1.0, 0.0]),
                           Vertex::new([0.5, 0.5, height], [1.0, 1.0]),
                           Vertex::new([-0.5, 0.5, height], [0.0, 1.0])];

    // Create the indices
    let index_data = [0, 1, 2, 2, 3, 0];

    // Scale the plane by the world scale and its size
    for pos in vertex_data.iter_mut().map(|v| &mut v.pos) {
        pos[0] *= WORLD_SCALE * size;
        pos[1] *= WORLD_SCALE * size;
    }

    (vertex_data, index_data)
}
