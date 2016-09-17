use hsgraphics::gfx3d::Vertex;
use consts::scale::WORLD_SCALE;

// TODO: Change return value to (array, array) to avoid allocations
// TODO: Add rotation
pub fn cube(position: [f32; 3], size: f32) -> (Vec<Vertex>, Vec<u16>)
{
    let mut vertex_data = vec![
        // top
        Vertex::new([-1.0, -1.0,  1.0], [0.0, 0.0]),
        Vertex::new([ 1.0, -1.0,  1.0], [1.0, 0.0]),
        Vertex::new([ 1.0,  1.0,  1.0], [1.0, 1.0]),
        Vertex::new([-1.0,  1.0,  1.0], [0.0, 1.0]),
        // bottom
        Vertex::new([-1.0,  1.0, -1.0], [1.0, 0.0]),
        Vertex::new([ 1.0,  1.0, -1.0], [0.0, 0.0]),
        Vertex::new([ 1.0, -1.0, -1.0], [0.0, 1.0]),
        Vertex::new([-1.0, -1.0, -1.0], [1.0, 1.0]),
        // right
        Vertex::new([ 1.0, -1.0, -1.0], [0.0, 0.0]),
        Vertex::new([ 1.0,  1.0, -1.0], [1.0, 0.0]),
        Vertex::new([ 1.0,  1.0,  1.0], [1.0, 1.0]),
        Vertex::new([ 1.0, -1.0,  1.0], [0.0, 1.0]),
        // left
        Vertex::new([-1.0, -1.0,  1.0], [1.0, 0.0]),
        Vertex::new([-1.0,  1.0,  1.0], [0.0, 0.0]),
        Vertex::new([-1.0,  1.0, -1.0], [0.0, 1.0]),
        Vertex::new([-1.0, -1.0, -1.0], [1.0, 1.0]),
        // front
        Vertex::new([ 1.0,  1.0, -1.0], [1.0, 0.0]),
        Vertex::new([-1.0,  1.0, -1.0], [0.0, 0.0]),
        Vertex::new([-1.0,  1.0,  1.0], [0.0, 1.0]),
        Vertex::new([ 1.0,  1.0,  1.0], [1.0, 1.0]),
        // back
        Vertex::new([ 1.0, -1.0,  1.0], [0.0, 0.0]),
        Vertex::new([-1.0, -1.0,  1.0], [1.0, 0.0]),
        Vertex::new([-1.0, -1.0, -1.0], [1.0, 1.0]),
        Vertex::new([ 1.0, -1.0, -1.0], [0.0, 1.0]),
    ];

    for pos in vertex_data.iter_mut().map(|v| &mut v.pos) {
        for (i, c) in pos.iter_mut().take(3).enumerate() {
            *c = position[i] * WORLD_SCALE + *c * WORLD_SCALE * size;
        }
    }

    let index_data = vec![
         0,  1,  2,  2,  3,  0, // top
         4,  5,  6,  6,  7,  4, // bottom
         8,  9, 10, 10, 11,  8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    (vertex_data, index_data)
}

pub fn plane(height: f32, size: f32) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertex_data = vec![
        Vertex::new([-1.0, -1.0,  height], [0.0, 0.0]),
        Vertex::new([ 1.0, -1.0,  height], [1.0, 0.0]),
        Vertex::new([ 1.0,  1.0,  height], [1.0, 1.0]),
        Vertex::new([-1.0,  1.0,  height], [0.0, 1.0]),
    ];
    
    let index_data = vec![0, 1, 2, 2, 3, 0];

    for pos in vertex_data.iter_mut().map(|v| &mut v.pos) {
        pos[0] *= WORLD_SCALE * size;
        pos[1] *= WORLD_SCALE * size;
    }

    (vertex_data, index_data)
}
