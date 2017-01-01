use consts::scale::WORLD_SCALE;
use hsgraphics::gfx3d::Vertex;

// TODO: Add rotation
pub fn cube(position: [f32; 3], size: f32) -> ([Vertex; 24], [u16; 36]) {
    let mut v = [[1.0, 1.0, 1.0],
                 [1.0, 1.0, -1.0],
                 [-1.0, 1.0, 1.0],
                 [-1.0, 1.0, -1.0],
                 [1.0, -1.0, 1.0],
                 [-1.0, -1.0, 1.0],
                 [1.0, -1.0, -1.0],
                 [-1.0, -1.0, -1.0]];

    for pos in &mut v {
        for (i, c) in pos[0..3].iter_mut().enumerate() {
            *c = position[i] * WORLD_SCALE + *c * WORLD_SCALE * size;
        }
    }

    // NOTE: Add rotation code before this to operate on a smaller array
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

    let index_data = [0, 1, 2, 2, 3, 0 /* top */, 4, 5, 6, 6, 7, 4 /* bottom */, 8, 9,
                      10, 10, 11, 8 /* right */, 12, 13, 14, 14, 15, 12 /* left */, 16,
                      17, 18, 18, 19, 16 /* front */, 20, 21, 22, 22, 23, 20 /* back */];

    (vertex_data, index_data)
}

pub fn plane(height: f32, size: f32) -> ([Vertex; 4], [u16; 6]) {
    let mut vertex_data = [Vertex::new([-0.5, -0.5, height], [0.0, 0.0]),
                           Vertex::new([0.5, -0.5, height], [1.0, 0.0]),
                           Vertex::new([0.5, 0.5, height], [1.0, 1.0]),
                           Vertex::new([-0.5, 0.5, height], [0.0, 1.0])];

    let index_data = [0, 1, 2, 2, 3, 0];

    for pos in vertex_data.iter_mut().map(|v| &mut v.pos) {
        pos[0] *= WORLD_SCALE * size;
        pos[1] *= WORLD_SCALE * size;
    }

    (vertex_data, index_data)
}
