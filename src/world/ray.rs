use std::iter::Iterator;
use world::Coords;

pub struct Ray {
    pub coords: Coords,
    pub interval: f64,
    pub direction: (f64, f64),
}

impl Iterator for Ray {
    type Item = Coords;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.coords.clone();
        self.coords.move_3d(self.direction, self.interval);
        Some(result)
    }
}
