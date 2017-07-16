//! Various math-related functions

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

#[cfg(test)]
mod tests {
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
}
