#[derive(Clone, Debug)]
pub struct Velocity {
    pub component_x: f64,
    pub component_y: f64,
}

impl Velocity {
    pub fn zero() -> Velocity {
        Velocity {
            component_x: 0.0,
            component_y: 0.0,
        }
    }

    pub fn accelerate(&mut self, accel_x: f64, accel_y: f64) {
        self.component_x += accel_x;
        self.component_y += accel_y;
    }
}
