//! Controls system to let players control their entity

use specs::{self, Join};
use cgmath::{self, Quaternion, Rotation3, Rad};

use std::sync::mpsc;

use world;
use control;
use player::components::Player;
use math::functions;

/// An event sent by a player, for example when a player presses a key an event will be generated
pub enum Event {
    /// A player rotated the camera (the direction will be added to the player entity's direction)
    RotateCamera(CameraRotation),
    MoveForward,
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

/// A type alias for convenience
type Euler = cgmath::Euler<Rad<::Float>>;

pub struct System {
    /// Receives events
    input: EventReceiver,
    /// The rotation to apply to the player entity
    rotate_direction: Option<CameraRotation>,
    /// Internally used for clamping the camera controls
    current_direction: Euler,
    /// Whether to move forward
    move_forward: bool,
}

impl System {
    pub fn new(input: EventReceiver) -> Self {
        Self {
            input: input,
            rotate_direction: None,
            current_direction: cgmath::Quaternion::from_angle_x(cgmath::Deg(0.0)).into(),
            move_forward: false,
        }
    }

    fn check_input(&mut self) {
        while let Ok(e) = self.input.try_recv() {
            match e {
                Event::RotateCamera(rot) => self.rotate_direction = Some(rot),
                Event::MoveForward => self.move_forward = true,
            }
        }
    }

    /// Applies the provided rotation to the current direction, and returns the new value
    fn update_direction(&mut self, rot: CameraRotation) -> Quaternion<::Float> {
        let current = &mut self.current_direction;

        // The pitch, yaw, and roll values are stored internally
        // Rotations are added to the stored values, and the rotation is constructed each
        // update, instead of accumulating
        current.x = functions::clamp(current.x + rot.pitch, Rad(0.0), Rad(3.14));
        current.y = functions::wrap(current.y + rot.yaw, Rad(-3.14), Rad(3.14));

        let pitch = Quaternion::from_angle_x(current.x);
        let yaw = Quaternion::from_angle_z(current.y);
        let pitch_yaw = yaw * pitch;

        let forward = pitch_yaw * cgmath::Vector3::unit_z();
        let roll = Quaternion::from_axis_angle(forward, current.z);

        roll * pitch_yaw
    }
}

#[derive(SystemData)]
pub struct Data<'a> {
    player: specs::ReadStorage<'a, Player>,
    control: specs::WriteStorage<'a, control::Control>,
    // Direction is directly accessed because it is special for the player (it is not tied to
    // physics)
    direction: specs::WriteStorage<'a, world::components::Direction>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // TODO: Maybe use delta time here for controls
        self.check_input();

        // Apply the input to the player entity
        for (d, c, _) in (&mut data.direction, &mut data.control, &data.player).join() {
            if let Some(rot) = self.rotate_direction.clone() {
                let new_direction = self.update_direction(rot);
                // Rotate the player entity's direction
                d.0 = new_direction;

                // Set the yaw of the player entity's physics body (ignoring the pitch and roll)
                c.set_rotation(Quaternion::from_angle_z(self.current_direction.y));
            }

            if self.move_forward {
                c.move_forward(1.0);
                self.move_forward = false;
            }
        }
    }
}
