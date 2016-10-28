use world::Coords;
use map::Map;

impl Map {
    pub fn test_collision(&self, coords: &Coords, entity_height: f64) -> bool {
        coords.y - entity_height <= self.floor_height
    }

    pub fn put_on_ground(&self, coords: &mut Coords, entity_height: f64) {
        coords.y = self.floor_height + entity_height;
    }
}
