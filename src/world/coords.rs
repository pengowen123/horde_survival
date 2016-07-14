use world::*;

#[derive(Clone, Debug)]
pub struct Coords {
    x: f64,
    y: f64,
    z: f64,
}

impl Coords {
    pub fn new(x: f64, y: f64, z: f64) -> Coords {
        Coords {
            x: x,
            y: y,
            z: z,
        }
    }
    
    pub fn origin() -> Coords {
        Coords::new(0.0, 0.0, 0.0)
    }

    pub fn distance(&self, other: &Coords) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)).powf(1.0 / 3.0)
    }
    
    pub fn in_radius(&self, other: &Coords, radius: f64) -> bool {
        self.distance(other) <= radius
    }

    pub fn translate(&mut self, other: &Coords) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn move_forward(&mut self, heading: f64, speed: f64) {
        let radians = Direction(heading).wrap().as_radians();
        let mut slope_x = radians.cos();
        let mut slope_z = radians.sin();

        normalize(&mut slope_x, &mut slope_z);

        slope_x *= speed;
        slope_z *= speed;

        self.translate(&Coords::new(slope_x, 0.0, slope_z))
    }

    pub fn move_3d(&mut self, direction: (f64, f64), speed: f64) {
        // use move_forward a second time to get the change in y-axis
    }
}
