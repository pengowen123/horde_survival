//! Conversion functions between `cgmath` and `nalgebra` types

use alga::general::Real;
use cgmath;
use na;

use std::ops::Index;

/// Returns a `cgmath::Point3` from a value that can be indexed
pub fn to_cgmath_point<P, N>(val: P) -> cgmath::Point3<N>
where
    N: cgmath::BaseNum,
    P: Index<usize, Output = N>,
{
    cgmath::Point3::new(val[0], val[1], val[2])
}

/// Returns a `cgmath::Vector3` from a value that can be indexed
pub fn to_cgmath_vector<V, N>(val: V) -> cgmath::Vector3<N>
where
    N: cgmath::BaseNum,
    V: Index<usize, Output = N>,
{
    cgmath::Vector3::new(val[0], val[1], val[2])
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
    na::UnitQuaternion::from_quaternion(na::Quaternion::new(quat.s, quat.v.x, quat.v.y, quat.v.z))
}

/// Returns a `nalgebra::Vector3` from the provided `cgmath::Vector3`
pub fn to_na_vector<T>(vector: cgmath::Vector3<T>) -> na::Vector3<T>
where
    T: Real,
{
    na::Vector3::new(vector[0], vector[1], vector[2])
}

/// Returns a `nalgebra::Point3` from the provided `cgmath::Point3`
pub fn to_na_point<T>(point: cgmath::Point3<T>) -> na::Point3<T>
where
    T: Real,
{
    na::Point3::new(point[0], point[1], point[2])
}

// TODO: Write tests
// TODO: Use quickcheck for all math tests
#[cfg(test)]
mod tests {
    use super::*;

    use cgmath::{Deg, InnerSpace, Rotation3};

    #[test]
    fn test_to_na_quaternion() {
        let quat = cgmath::Quaternion::from_axis_angle(
            cgmath::Vector3::new(1.0, 1.0, 1.0).normalize(),
            Deg(45.0),
        );
        let na_quat = to_na_quaternion(quat);

        let cgmath_output = quat * cgmath::Vector3::unit_z();
        let na_output = na_quat * na::Vector3::z();

        assert_eq!(to_na_vector(cgmath_output), na_output);
    }
}
