//! A system to update the camera


// TODO: Fix objects disappearing when the camera points straight up or down

use specs::{self, ReadStorage, Join};
use cgmath;

use world;
use player;
use graphics::window;
use math::direction;

/// Vertical field of view of the camera
const FOV_Y: f32 = 45.0;

/// Near plane distance for the cull frustum
const NEAR: f32 = 0.01;

/// Far plane distance for the cull frustum (controls render distance)
const FAR: f32 = 1000.0;

/// Represents a camera in a 3D space
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    matrix: [[f32; 4]; 4],
}

impl Camera {
    /// Returns a camera, with the eye at the provided position, pointing in the provided position.
    /// Requires the aspect ratio of the window the camera will be used with.
    pub fn new(pos: world::Position, direction: direction::Direction, aspect_ratio: f32) -> Self {
        let pos = pos.cast::<f32>();
        // TODO: Delete this line when movement controls and physics are complete
        let pos = cgmath::Point3::new(5.0, 5.0, 5.0);
        let pointing_to = pos + direction.into_vector().cast::<f32>();
        let view = cgmath::Matrix4::look_at(pos, pointing_to, cgmath::Vector3::unit_z());
        let proj = cgmath::perspective(cgmath::Deg(FOV_Y), aspect_ratio, NEAR, FAR);

        Self { matrix: (proj * view).into() }
    }

    /// Returns the default camera given the aspect ratio
    pub fn new_default(aspect_ratio: f32) -> Self {
        Camera::new(cgmath::Point3::new(0.0, 0.0, 0.0),
                    direction::Direction::new(cgmath::Rad(0.0), cgmath::Rad(0.0)),
                    aspect_ratio)
    }

    pub fn matrix(&self) -> [[f32; 4]; 4] {
        self.matrix
    }
}


pub struct System;

#[derive(SystemData)]
pub struct Data<'a> {
    _player: ReadStorage<'a, player::Player>,
    space: ReadStorage<'a, world::Spatial>,
    direction: ReadStorage<'a, world::Direction>,
    camera: specs::FetchMut<'a, Camera>,
    window_info: specs::Fetch<'a, window::WindowInfo>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // NOTE: This loop should only run for one iteration (there should only be one player
        //       entity locally)
        for (i, (space, direction)) in (&data.space, &data.direction).join().enumerate() {
            assert!(i == 0, "Found multiple player entities");

            *data.camera =
                Camera::new(space.position, direction.0, data.window_info.aspect_ratio());
        }
    }
}
