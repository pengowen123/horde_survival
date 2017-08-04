//! A resource to store the camera, and a system to update it

use specs::{self, ReadStorage, Join};
use cgmath::{self, Rotation3, EuclideanSpace, SquareMatrix};

use world;
use math::functions;
use player::components::Player;
use window::info;

/// Vertical field of view of the camera
const FOV_Y: f32 = 45.0;

/// Near plane distance for the cull frustum
const NEAR: f32 = 1.0;

/// Far plane distance for the cull frustum
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

    /// Calculates the `view * projection` matrix for the skybox camera
    ///
    /// This matrix has translation removed because the skybox should stay centered on the camera.
    /// It is also rotated to account for coordinate space differences.
    pub fn skybox_camera(&self) -> cgmath::Matrix4<f32> {
        // Remove the translation
        let mut view = functions::remove_translation(self.view);
        // Rotate the camera
        view = view * cgmath::Matrix4::from_angle_x(cgmath::Deg(90.0));

        // Create the camera matrix
        self.proj * view
    }

    /// Returns the eye position
    pub fn eye_position(&self) -> cgmath::Point3<f32> {
        self.eye_position
    }
}


pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    player: ReadStorage<'a, Player>,
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
