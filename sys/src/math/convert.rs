//! Conversion functions between `cgmath` and `nalgebra` types

use na;
use cgmath;

/// Creates and returns a `cgmath::Point3` from the provided `nalgebra::Point3`
pub fn to_cgmath_point(point: na::Point3<::Float>) -> cgmath::Point3<::Float> {
    let mut iter = point.iter();
    let x = *iter.next().unwrap();
    let y = *iter.next().unwrap();
    let z = *iter.next().unwrap();

    cgmath::Point3::new(x, y, z)
}

/// Creates and returns a `cgmath::Matrix4` rotation matrix from the provided
/// `nalgebra::QuaternionBase`
pub fn to_cgmath_quaternion<S>(quat: na::QuaternionBase<::Float, S>) -> cgmath::Quaternion<::Float>
where
    S: na::storage::Storage<::Float, na::U4, na::U1>,
{
    let scalar = quat.scalar();
    let cgmath_vec = cgmath::Vector3::new(quat[0], quat[1], quat[2]);
    cgmath::Quaternion::from_sv(scalar, cgmath_vec)
}
