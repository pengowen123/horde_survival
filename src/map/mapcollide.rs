//! Collision detection for maps

use world::Coords;
use map::Map;

impl Map {
    /// Returns whether the entity with the given coordinates and height collide with the map

    // TODO: This will be significantly more complex when real collision detection is implemented
    pub fn test_collision(&self, coords: &Coords, entity_height: f64) -> bool {
        coords.y - entity_height <= self.floor_height
    }

    /// Puts the coordinates on the ground given an entity height
    pub fn put_on_ground(&self, coords: &mut Coords, entity_height: f64) {
        coords.y = self.floor_height + entity_height;
    }
}
