#[macro_use]
pub mod misc;
pub mod balance;
pub mod controls;
pub mod graphics;
pub mod physics;
pub mod log_str;
pub mod scale;
pub mod defaults;
pub mod shader;

pub use self::balance::*;
pub use self::controls::*;
pub use self::graphics::*;
pub use self::misc::*;
pub use self::physics::*;
pub use self::scale::*;
pub use self::shader::*;

// NOTE: Delete this when spawn points can be loaded from map files
use world::Coords;

pub const TEST_SPAWN_POINTS: ([Coords; 2], [f64; 2]) = (
    [coords!(5.0, 0.0, 5.0), coords!(-5.0, 0.0, -5.0)],
    [1.0, 1.0]
);
