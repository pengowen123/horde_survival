//! Conversion functions between `cgmath` and `nalgebra` types

use na;
use cgmath;

/// Creates and returns a `cgmath::Point3` from the provided `nalgebra::Point3`
pub fn to_cgmath_point(point: na::Point3<::Float>) -> cgmath::Point3<::Float> {
    cgmath::Point3::new(point[0], point[1], point[2])
}

/// Creates and returns a `cgmath::Matrix4` rotation matrix from the provided
/// `nalgebra::QuaternionBase`
pub fn to_cgmath_quaternion<S>(
    quat: na::UnitQuaternionBase<::Float, S>,
) -> cgmath::Quaternion<::Float>
where
    S: na::storage::Storage<::Float, na::U4, na::U1>,
{
    let scalar = quat.scalar();
    let cgmath_vec = cgmath::Vector3::new(quat[0], quat[1], quat[2]);
    cgmath::Quaternion::from_sv(scalar, cgmath_vec)
}

/// Creates and returns a `nalgebra::UnitQuaternionBase` from the provided `cgmath::Quaternion`
pub fn to_na_quaternion(
    quat: cgmath::Quaternion<::Float>,
) -> na::UnitQuaternionBase<::Float, na::MatrixArray<::Float, na::U4, na::U1>> {
    na::UnitQuaternionBase::from_quaternion(na::QuaternionBase::new(
        quat.s,
        quat.v[0],
        quat.v[1],
        quat.v[2],
    ))
}
