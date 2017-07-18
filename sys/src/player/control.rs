//! Controls system to let players control their entity

use specs::{self, Join};
use cgmath::{self, Quaternion, Rotation3, Rad};

use std::sync::mpsc;

use world;
use player::components::Player;
use math::functions;

/// An event sent by a player, for example when a player presses a key an event will be generated
pub enum Event {
    /// A player rotated the camera (the direction will be added to the player entity's direction)
    RotateCamera(CameraRotation),
}

pub type EventReceiver = mpsc::Receiver<Event>;

#[derive(Clone, Copy, Debug)]
pub struct CameraRotation {
    pitch: Rad<::Float>,
    yaw: Rad<::Float>,
}

impl CameraRotation {
    pub fn new<T: Into<Rad<::Float>>>(pitch: T, yaw: T) -> Self {
        Self {
            pitch: pitch.into(),
            yaw: yaw.into(),
        }
    }
}

pub type CameraDirection = cgmath::Euler<Rad<::Float>>;

pub struct System {
    input: EventReceiver,
    rotate_direction: Option<CameraRotation>,
    current_direction: CameraDirection,
}

impl System {
    pub fn new(input: EventReceiver) -> Self {
        Self {
            input: input,
            rotate_direction: None,
            current_direction: cgmath::Quaternion::from_angle_x(cgmath::Deg(0.0)).into(),
        }
    }

    fn check_input(&mut self) {
        while let Ok(e) = self.input.try_recv() {
            match e {
                Event::RotateCamera(rot) => {
                    self.rotate_direction = Some(rot);
                }
            }
        }
    }
}

#[derive(SystemData)]
pub struct Data<'a> {
    player: specs::ReadStorage<'a, Player>,
    direction: specs::WriteStorage<'a, world::components::Direction>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // TODO: Maybe use delta time here for controls
        self.check_input();

        // Apply the input to the player entity
        for (d, _) in (&mut data.direction, &data.player).join() {
            if let Some(rot) = self.rotate_direction.clone() {
                let current = &mut self.current_direction;

                // The pitch, yaw, and roll values are stored internally
                // Rotations are added to the stored values, and the rotation is constructed each
                // update, instead of accumulating
                current.x = functions::clamp(current.x + rot.pitch, Rad(0.0), Rad(3.14));
                current.y = functions::wrap(current.y + rot.yaw, Rad(-3.14), Rad(3.14));

                let x = current.x;
                let y = current.y;
                let z = current.z;

                let pitch = Quaternion::from_angle_x(x);
                let yaw = Quaternion::from_angle_z(y);
                let pitch_yaw = yaw * pitch;

                let forward = pitch_yaw * cgmath::Vector3::unit_z();
                let roll = Quaternion::from_axis_angle(forward, z);

                let quat = roll * pitch_yaw;

                // Rotate the player entity's direction
                d.0 = quat;
            }
        }
    }
}
