use world::*;
use map::Map;

impl Map {
    pub fn test_collision(&self, coords: &Coords, height: f64) -> bool {
        coords.y - height <= self.floor_height
    }

    pub fn put_on_ground(&self, coords: &mut Coords, height: f64) {
        coords.y = self.floor_height + height;
    }
}
