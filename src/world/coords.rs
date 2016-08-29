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

    pub fn move_forward(&mut self, heading: f64, distance: f64) {
        let radians = Direction(heading).wrap().as_radians();
        let mut slope_x = radians.cos();
        let mut slope_z = radians.sin();

        normalize(&mut slope_x, &mut slope_z);

        slope_x *= distance;
        slope_z *= distance;

        self.translate(&Coords::new(slope_x, 0.0, slope_z))
    }

    pub fn move_3d(&mut self, direction: (f64, f64), distance: f64) {
        let angle_x = Direction(direction.0).wrap().as_radians();
        let angle_y = Direction(direction.1).wrap().as_radians();

        let x = distance * angle_y.cos() * angle_x.sin();
        let y = distance * angle_x.cos();
        let z = distance * angle_y.sin() * angle_x.sin();

        self.translate(&Coords::new(x, y, z));
    }

    pub fn ray(&self, interval: f64, direction: (f64, f64)) -> Ray {
        Ray {
            coords: self.clone(),
            interval: interval,
            direction: direction,
        }
    }
}
