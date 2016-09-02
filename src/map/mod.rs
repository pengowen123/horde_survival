pub mod collision;

pub use self::collision::*;

pub struct Map {
    pub floor_height: f64,
}

impl Map {
    pub fn new(floor_height: f64) -> Map {
        Map {
            floor_height: floor_height,
        }
    }
}
