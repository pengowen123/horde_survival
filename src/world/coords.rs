use world::*;
use cgmath::{Vector3, Point3};

/// 3d coordinates that represent a point
#[derive(Clone, Debug, Default, Copy)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Coords {
    pub fn new(x: f64, y: f64, z: f64) -> Coords {
        Coords { x: x, y: y, z: z }
    }
}

impl Coords {
    /// Returns the distance from this point to another
    pub fn distance(&self, other: &Coords) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    /// Returns whether this point in within a radius to the other point
    pub fn in_radius(&self, other: &Coords, radius: f64) -> bool {
        self.distance(other) <= radius
    }

    /// Scales the point in-place
    pub fn scale(&mut self, factor: f64) {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }

    /// Translates the point in-place
    pub fn translate(&mut self, other: &Coords) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    /// Moves the point in a direction
    /// Only the x and y are moved (in a 2d space)
    pub fn move_forward(&mut self, heading: f64, distance: f64) {
        let radians = Direction(heading).wrap().as_radians();
        let mut slope_x = radians.cos();
        let mut slope_z = radians.sin();

        normalize(&mut slope_x, &mut slope_z);

        slope_x *= distance;
        slope_z *= distance;

        self.translate(&Coords::new(slope_x, 0.0, slope_z))
    }

    /// Moves the point in a direction (in a 3d space)

    // NOTE: Apparently cgmath has some rotation methods, use those instead
    pub fn move_3d(&mut self, direction: (f64, f64), distance: f64) {
        let angle_x = Direction(direction.0).wrap().as_radians();
        let angle_y = Direction(direction.1).wrap().as_radians();

        let x = distance * angle_y.cos() * angle_x.sin();
        let y = distance * angle_x.cos();
        let z = distance * angle_y.sin() * angle_x.sin();

        self.translate(&Coords::new(x, y, z));
    }

    /// Returns a ray with the given direction and interval (see docs for Ray)
    pub fn ray(&self, interval: f64, direction: (f64, f64)) -> Ray {
        Ray {
            coords: self.clone(),
            interval: interval,
            direction: direction,
        }
    }

    /// Returns the direction from this point to another
    pub fn direction_to(&self, other: &Coords) -> (f64, f64) {
        let rise = self.y - other.y;
        let run = (self.x - other.x).abs();
        let rot_x = get_angle(rise, run);

        let rise = self.x - other.x;
        let run = self.z - other.z;
        let rot_y = get_angle2(run, rise);

        (rot_x, rot_y)
    }

    /// Returns this point, translated in the given direction
    pub fn translated(&self, x: f64, y: f64, z: f64) -> Coords {
        let mut coords = self.clone();
        coords.x += x;
        coords.y += y;
        coords.z += z;
        coords
    }
}

impl Into<Vector3<f64>> for Coords {
    fn into(self) -> Vector3<f64> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl Into<Point3<f64>> for Coords {
    fn into(self) -> Point3<f64> {
        Point3::new(self.x, self.y, self.z)
    }
}

/// Creates a Coords from an x, y, and z
macro_rules! coords {
    ($x:expr, $y:expr, $z:expr) => {{
        $crate::world::Coords {
            x: $x,
            y: $y,
            z: $z,
        }
    }}
}
