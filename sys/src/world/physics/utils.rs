//! Utility functions for physics (mostly conversions to/from `cgmath` types for `nalgebra` types)

use nphysics3d;
use cgmath;

pub fn to_cgmath_point(point: nphysics3d::math::Point<::Float>) -> cgmath::Point3<::Float> {
    let mut iter = point.iter();
    let x = *iter.next().unwrap();
    let y = *iter.next().unwrap();
    let z = *iter.next().unwrap();

    cgmath::Point3::new(x, y, z)
}
