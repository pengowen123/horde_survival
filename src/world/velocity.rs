/// A velocity consisting of an x and y component
#[derive(Clone, Debug, Default)]
pub struct Velocity {
    pub component_x: f64,
    pub component_y: f64,
}

impl Velocity {
    /// Accelerates the velocity
    pub fn accelerate(&mut self, accel_x: f64, accel_y: f64) {
        self.component_x += accel_x;
        self.component_y += accel_y;
    }
}
