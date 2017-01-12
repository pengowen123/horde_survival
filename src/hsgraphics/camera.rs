use cgmath::{self, Point3, Vector3, Matrix4, Transform};

use world::Coords;
use consts::{VERTICAL_FOV, WORLD_SCALE, START_CAMERA_ANGLE};

/// A camera
#[derive(Clone, Copy)]
pub struct Camera {
    pub direction: (f64, f64),
    pub coords: Coords,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            direction: START_CAMERA_ANGLE,
            coords: Default::default(),
        }
    }
}

impl Camera {
    /// Returns the camera as a 4x4 matrix given the aspect ratio
    pub fn into_matrix(self, aspect_ratio: f32) -> Matrix4<f32> {
        let Camera { mut coords, direction } = self;
        coords.scale(WORLD_SCALE as f64);
        let mut pointing_to = coords.clone();
        pointing_to.move_3d(direction, 1.0);

        let camera_pos = Point3::new(coords.x as f32, coords.z as f32, coords.y as f32);
        let pointing_to = Point3::new(pointing_to.x as f32,
                                      pointing_to.z as f32,
                                      pointing_to.y as f32);

        let view: Matrix4<f32> = Transform::look_at(camera_pos, pointing_to, Vector3::unit_z());

        let proj = cgmath::perspective(cgmath::Deg(VERTICAL_FOV), aspect_ratio, 0.01, 100.0);

        proj * view
    }
}

/// Returns the initial camera, given the player spawn point and the aspect ratio
pub fn initial_camera(coords: Coords, aspect_ratio: f32) -> Matrix4<f32> {
    Camera {
            coords: coords,
            direction: START_CAMERA_ANGLE,
        }
        .into_matrix(aspect_ratio)
}
