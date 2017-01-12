//! Types and functions for dealing with 3d spaces

// TODO: Replace this module with a small wrapper for a third party library

#[macro_use]
pub mod coords;
pub mod direction;
pub mod vector;
pub mod ray;
pub mod velocity;

pub use self::coords::Coords;
pub use self::direction::*;
pub use self::vector::*;
pub use self::ray::Ray;
pub use self::velocity::Velocity;
