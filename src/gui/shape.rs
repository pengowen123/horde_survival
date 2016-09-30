use collision::{Aabb, Aabb2};
use cgmath::{EuclideanSpace, Vector2};

pub trait UIShape {
    fn dimensions(&self) -> [f32; 2];
    fn set_position(&mut self, position: [f32; 2]);
}

impl UIShape for Aabb2<f32> {
    fn dimensions(&self) -> [f32; 2] {
        [self.dim().x, self.dim().y]
    }

    fn set_position(&mut self, position: [f32; 2]) {
        *self = self.add_v(-self.center().to_vec());
        let dim = self.dimensions();
        let (dim_x, dim_y) = (dim[0], dim[1]);
        let center = Vector2::new(dim_x / 2.0, dim_y / 2.0);
        let displacement = Vector2::new(position[0], position[1]);

        *self = self.add_v(center + displacement);
    }
}
