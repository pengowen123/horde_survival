pub mod balance;
pub mod controls;
pub mod graphics;
pub mod misc;
pub mod physics;
pub mod log_str;
pub mod scale;
pub mod text;
pub mod defaults;

pub use self::balance::*;
pub use self::controls::*;
pub use self::graphics::*;
pub use self::misc::*;
pub use self::physics::*;
pub use self::scale::*;

// NOTE: Delete this when spawn points can be loaded from map files
use world::Coords;

pub const TEST_SPAWN_POINTS: ([Coords; 2], [f64; 2]) = (
    [Coords::new(5.0, 0.0, 5.0), Coords::new(-5.0, 0.0, -5.0)],
    [1.0, 1.0]
);
