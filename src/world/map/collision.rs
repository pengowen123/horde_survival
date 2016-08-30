use world::*;

impl Map {
    pub fn test_collision(&self, coords: &Coords) -> bool {
        coords.y <= self.floor_height
    }

    pub fn put_on_ground(&self, coords: &mut Coords) {
        coords.y = self.floor_height;
    }
}
