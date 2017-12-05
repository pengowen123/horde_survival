//! Various math-related functions

use cgmath::{self, InnerSpace};

use std::ops;

/// Returns `val`, clamped to the range between `lower` and `upper`
pub fn clamp<T: PartialOrd>(val: T, lower: T, upper: T) -> T {
    if val < lower {
        return lower;
    } else if val > upper {
        return upper;
    }

    val
}

/// Returns `val`, wrapped in the range between `lower` and `upper`
///
/// This is similar to the modulus operator `%`, but works with ranges from negative to positive
/// numbers
pub fn wrap<T>(val: T, lower: T, upper: T) -> T
where
    T: Clone + PartialOrd + ops::Sub<Output = T> + ops::Add<Output = T>,
{
    let mut val = val;
    let diff = upper.clone() - lower.clone();

    while val > upper {
        let old = val.clone();
        val = val - diff.clone();

        // If `diff` doesn't affect `val`, the range has zero size, so this will loop forever
        // without this
        if val == old {
            val = upper;
            return val;
        }
    }

    while val < lower {
        let old = val.clone();
        val = val + diff.clone();

        // If `diff` doesn't affect `val`, the range has zero size, so this will loop forever
        // without this
        if val == old {
            val = lower;
            return val;
        }
    }

    val
}

/// Returns the matrix with the translation removed
pub fn remove_translation<T>(mat: cgmath::Matrix4<T>) -> cgmath::Matrix4<T>
where
    T: cgmath::BaseFloat,
{
    let row0 = cgmath::Vector3::new(mat.x.x, mat.x.y, mat.x.z);
    let row1 = cgmath::Vector3::new(mat.y.x, mat.y.y, mat.y.z);
    let row2 = cgmath::Vector3::new(mat.z.x, mat.z.y, mat.z.z);

    cgmath::Matrix3 {
        x: row0,
        y: row1,
        z: row2,
    }.into()
}

/// Returns the direction vector represented as a quaternion rotation from the unit `z` vector
pub fn dir_vec_to_quaternion<T, N>(vec: T) -> cgmath::Quaternion<N>
where
    T: Into<cgmath::Vector3<N>>,
    N: cgmath::BaseFloat,
{
    let d: cgmath::Vector3<N> = vec.into();
    let d = d.normalize();

    cgmath::Quaternion::from_arc(cgmath::Vector3::unit_z(), d, None)
}

#[cfg(test)]
mod tests {
    use cgmath::*;

    use super::*;

    #[test]
    fn test_clamp_in_range() {
        let val = 3;
        let clamped = clamp(val, 0, 6);
        assert_eq!(clamped, val);
    }

    #[test]
    fn test_clamp_lower() {
        let val = -2;
        let clamped = clamp(val, 0, 6);
        assert_eq!(clamped, 0);
    }

    #[test]
    fn test_clamp_upper() {
        let val = 9;
        let clamped = clamp(val, 0, 6);
        assert_eq!(clamped, 6);
    }

    #[test]
    fn test_wrap_in_range() {
        let val = 3;
        let wrapped = wrap(val, 0, 6);
        assert_eq!(wrapped, 3);
    }

    #[test]
    fn test_wrap_lower() {
        let val = -4;
        let wrapped = wrap(val, 0, 6);
        assert_eq!(wrapped, 2);
    }

    #[test]
    fn test_wrap_upper() {
        let val = 10;
        let wrapped = wrap(val, 0, 6);
        assert_eq!(wrapped, 4);
    }

    #[test]
    fn test_wrap_zero_size_range() {
        let val = 10;
        let wrapped = wrap(val, 5, 5);
        assert_eq!(wrapped, 5);

        let val = 0;
        let wrapped = wrap(val, 5, 5);
        assert_eq!(wrapped, 5);
    }

    #[test]
    fn test_remove_translation() {
        let rotate = Matrix4::from_angle_y(Deg(180.0));
        let translate = Matrix4::from_translation(Vector3::new(0.0, 0.0, 10.0));

        let vec = Vector4::new(1.0, 0.0, 0.0, 1.0);

        let transform = translate * rotate;

        // The vector is rotated 180 degrees then translated by 10 units in the positive Z
        // direction
        assert_relative_eq!(transform * vec, Vector4::new(-1.0, 0.0, 10.0, 1.0));

        // With the translation removed, the vector should only be rotated 180 degrees
        let removed = remove_translation(transform);
        assert_relative_eq!(removed * vec, Vector4::new(-1.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn test_dir_vec_to_quaternion() {
        let vectors = [
            (1.0, 0.0, 0.0),
            (0.0, 1.0, 0.0),
            (0.0, 0.0, 1.0),
            (0.5, 0.3, 1.0),
            (0.1, 0.6, 1.2),
        ];

        for vec in &vectors {
            let vec = Vector3::new(vec.0, vec.1, vec.2).normalize();
            let quat = dir_vec_to_quaternion(vec);

            // The quaternion turned into a vector should equal the original direction vector
            assert_relative_eq!(quat * Vector3::unit_z(), vec);
        }
    }
}
