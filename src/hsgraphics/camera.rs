use cgmath::{self, Point3, Vector3, Matrix4, Transform};

use world::Coords;
use consts::graphics::*;

pub fn get_camera(mut coords: Coords, direction: (f64, f64), aspect_ratio: f32) -> [[f32; 4]; 4] {
    let mut pointing_to = coords.clone();
    pointing_to.move_3d(direction, 1.0);

    let camera_pos = Point3::new(coords.x as f32, coords.z as f32, coords.y as f32);
    let pointing_to = Point3::new(pointing_to.x as f32, pointing_to.z as f32, pointing_to.y as f32);

    let view: Matrix4<f32> = Transform::look_at(
        camera_pos,
        pointing_to,
        Vector3::unit_z(),
    );

    let proj = cgmath::perspective(cgmath::Deg(VERTICAL_FOV), aspect_ratio, 0.01, 100.0);

    (proj * view).into()
}
