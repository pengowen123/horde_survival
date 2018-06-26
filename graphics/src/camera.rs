//! A resource to store the camera, and a system to update it

use common::specs::{self, ReadStorage, Join};
use common::cgmath::{self, Rotation3, EuclideanSpace, SquareMatrix};
use common;
use math::functions;
use window::info;

use std::sync::{Arc, Mutex};

/// Vertical field of view of the camera
const FOV_Y: f32 = 45.0;

/// Near plane distance for the camera
// NOTE: This must not be greater than the near plane value used for light shadows, or instead of
//       disappearing, close-by objects will appear completely black
const NEAR: f32 = 0.1;

/// Far plane distance for the camera
const FAR: f32 = 1000.0;

/// Represents a camera in a 3D space
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    /// The projection matrix
    proj: cgmath::Matrix4<f32>,
    /// The view matrix
    view: cgmath::Matrix4<f32>,
    /// The position of the eye
    eye_position: cgmath::Point3<f32>,
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
        let pos_vec = pos.to_vec();
        let view = (cgmath::Matrix4::from_translation(pos_vec) * cgmath::Matrix4::from(direction))
            .invert()
            .unwrap();

        Self {
            proj,
            view,
            eye_position: pos,
        }
    }

    /// Returns the default camera given the aspect ratio
    pub fn new_default(aspect_ratio: f32) -> Self {
        Camera::new(
            cgmath::Point3::new(0.0, 0.0, 0.0),
            cgmath::Quaternion::from_angle_x(cgmath::Deg(0.0)),
            aspect_ratio,
        )
    }

    /// Returns the projection matrix
    pub fn projection(&self) -> cgmath::Matrix4<f32> {
        self.proj
    }

    /// Returns the view matrix
    pub fn view(&self) -> cgmath::Matrix4<f32> {
        self.view
    }

    /// Calculates the `view` matrix for the skybox camera
    /// 
    /// It is rotated to account for coordinate space differences.
    pub fn skybox_view(&self) -> cgmath::Matrix4<f32> {
        self.view * cgmath::Matrix4::from_angle_x(cgmath::Deg(90.0))
    }

    /// Returns the eye position
    pub fn eye_position(&self) -> cgmath::Point3<f32> {
        self.eye_position
    }
}


pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    player: ReadStorage<'a, common::Player>,
    space: ReadStorage<'a, common::Position>,
    direction: ReadStorage<'a, common::Direction>,
    camera: specs::FetchMut<'a, Arc<Mutex<Camera>>>,
    window_info: specs::Fetch<'a, info::WindowInfo>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, data: Self::SystemData) {
        for (s, d, _) in (&data.space, &data.direction, &data.player).join() {
            *data.camera.lock().unwrap() = Camera::new(
                s.0.cast(),
                d.0.cast(),
                data.window_info.aspect_ratio(),
            );
        }
    }
}
