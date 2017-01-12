/// Normalizes the x and y values of a vector
pub fn normalize(x: &mut f64, y: &mut f64) {
    let magnitude = magnitude(*x, *y);
    *x /= magnitude;
    *y /= magnitude;
}

/// Returns the magnitude of the vector
pub fn magnitude(x: f64, y: f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()
}
