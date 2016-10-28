pub fn normalize(x: &mut f64, y: &mut f64) {
    let magnitude = magnitude(*x, *y);
    *x /= magnitude;
    *y /= magnitude;
}

pub fn magnitude(x: f64, y: f64) -> f64 {
    (x.powi(2) + y.powi(2)).sqrt()
}
