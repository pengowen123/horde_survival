//! Conversion functions between `cgmath` and `nalgebra` types

use na;
use cgmath;
use alga::general::Real;

/// Returns a `cgmath::Point3` from the provided `nalgebra::Point3`
pub fn to_cgmath_point<T>(point: na::Point3<T>) -> cgmath::Point3<T>
where
    T: cgmath::BaseNum + Real,
{
    cgmath::Point3::new(point[0], point[1], point[2])
}

/// Returns a `cgmath::Matrix4` rotation matrix from the provided
/// `nalgebra::QuaternionBase`
pub fn to_cgmath_quaternion<T>(quat: na::UnitQuaternion<T>) -> cgmath::Quaternion<T>
where
    T: cgmath::BaseFloat + Real,
{
    let scalar = quat.scalar();
    let cgmath_vec = cgmath::Vector3::new(quat[0], quat[1], quat[2]);
    cgmath::Quaternion::from_sv(scalar, cgmath_vec)
}

/// Returns a `nalgebra::UnitQuaternion` from the provided `cgmath::Quaternion`
pub fn to_na_quaternion<T>(quat: cgmath::Quaternion<T>) -> na::UnitQuaternion<T>
where
    T: Real,
{
    na::UnitQuaternion::from_quaternion(na::QuaternionBase::new(
        quat.s,
        quat.v[0],
        quat.v[1],
        quat.v[2],
    ))
}

/// Returns a `nalgebra::Vector3` from the provided `cgmath::Vector3`
pub fn to_na_vector<T>(vector: cgmath::Vector3<T>) -> na::Vector3<T>
where
    T: Real,
{
    na::Vector3::new(vector[0], vector[1], vector[2])
}
