pub mod mapcollide;

pub use self::mapcollide::*;

use random_choice::random_choice;

use world::Coords;

pub struct Map {
    pub floor_height: f64,
    pub spawn_points: (Vec<Coords>, Vec<f64>),
    pub player_spawn: Coords,
}

impl Map {
    pub fn new(floor_height: f64, player_spawn: Coords, spawn_points: (Vec<Coords>, Vec<f64>)) -> Map {
        Map {
            floor_height: floor_height,
            spawn_points: spawn_points,
            player_spawn: player_spawn,
        }
    }
}

impl Map {
    pub fn choose_random_spawn_point(&self) -> Coords {
        let (points, weights) = (&self.spawn_points.0, &self.spawn_points.1);
        let choice = random_choice().random_choice_f64(points, weights, 1);

        choice[0].clone()
    }
}
