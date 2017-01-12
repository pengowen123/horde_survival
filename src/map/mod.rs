//! Everything related to maps

pub mod mapcollide;
#[macro_use]
pub mod spawn;

pub use self::mapcollide::*;
pub use self::spawn::*;

use world::Coords;

/// A map
pub struct Map {
    pub floor_height: f64,
    pub spawn_points: Vec<SpawnPoint>,
    pub player_spawn: Coords,
}

impl Map {
    pub fn new(floor_height: f64, player_spawn: Coords, spawn_points: Vec<SpawnPoint>) -> Map {
        Map {
            floor_height: floor_height,
            spawn_points: spawn_points,
            player_spawn: player_spawn,
        }
    }
}
