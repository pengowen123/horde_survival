//! A resource to store the camera, and a system to update it

// TODO: Fix objects disappearing when the camera points straight up or down

use specs::{self, ReadStorage, Join};
use cgmath::{self, Rotation3, EuclideanSpace, SquareMatrix};

use world;
use player;
use window::info;

/// Vertical field of view of the camera
const FOV_Y: f32 = 45.0;

/// Near plane distance for the cull frustum
const NEAR: f32 = 0.01;

/// Far plane distance for the cull frustum (controls render distance)
const FAR: f32 = 1000.0;

/// Represents a camera in a 3D space
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    /// The camera matrix (Projection * View)
    matrix: cgmath::Matrix4<f32>,
}

impl Camera {
    /// Returns a camera, with the eye at the provided position, pointing in the provided direction.
    /// Requires the aspect ratio of the window the camera will be used with.
    pub fn new(
        pos: cgmath::Point3<f32>,
        direction: cgmath::Quaternion<f32>,
        aspect_ratio: f32,
    ) -> Self {

        let proj = cgmath::perspective(cgmath::Deg(FOV_Y), aspect_ratio, NEAR, FAR);
        let pos = pos.to_vec();
        let view = (cgmath::Matrix4::from_translation(pos) * cgmath::Matrix4::from(direction))
            .invert()
            .unwrap();

        Self { matrix: (proj * view).into() }
    }

    /// Returns the default camera given the aspect ratio
    pub fn new_default(aspect_ratio: f32) -> Self {
        Camera::new(
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Quaternion::from_angle_x(cgmath::Deg(0.0)),
            aspect_ratio,
        )
    }

    /// Returns the camera matrix
    pub fn get_matrix(&self) -> &cgmath::Matrix4<f32> {
        &self.matrix
    }
}


pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    player: ReadStorage<'a, player::Player>,
    space: ReadStorage<'a, world::components::Spatial>,
    direction: ReadStorage<'a, world::components::Direction>,
    camera: specs::FetchMut<'a, Camera>,
    window_info: specs::Fetch<'a, info::WindowInfo>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        for (s, d, _) in (&data.space, &data.direction, &data.player).join() {
            *data.camera = Camera::new(
                s.0.cast(),
                cgmath::Quaternion::from_sv(d.0.s as f32, d.0.v.cast()),
                data.window_info.aspect_ratio(),
            );
        }
    }
}
