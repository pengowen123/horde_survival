//! Various math-related functions

pub fn clamp<T: PartialOrd>(val: T, lower: T, upper: T) -> T {
    if val < lower {
        return lower;
    } else if val > upper {
        return upper;
    }

    val
}
