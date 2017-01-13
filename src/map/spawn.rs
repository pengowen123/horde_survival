use random_choice::random_choice;

use world::Coords;
use map::Map;

impl Map {
    /// Returns a random point to spawn the entity in, generated from an internal spawnpoint list
    pub fn choose_random_spawn_point(&self) -> Coords {
        let (points, weights) = self.spawn_points
            .iter()
            .map(|p| (p.coords, p.weight))
            .unzip::<_, _, Vec<_>, Vec<_>>();
        let choice = random_choice().random_choice_f64(&points, &weights, 1);

        *choice[0]
    }
}

/// A spawnpoint in a map
#[derive(Clone)]
pub struct SpawnPoint {
    pub coords: Coords,
    pub weight: f64,
}

/// Creates a SpawnPoint
macro_rules! spawnpoint {
    ($coords:expr, $weight:expr) => {{
        SpawnPoint {
            coords: $coords,
            weight: $weight,
        }
    }};
}
