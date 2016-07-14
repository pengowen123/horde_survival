pub fn normalize(x: &mut f64, y: &mut f64) {
    let magnitude = (x.powi(2) + y.powi(2)).sqrt();
    *x /= magnitude;
    *y /= magnitude;
}
