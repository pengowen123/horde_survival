//! Controls system to let players control their entity

pub mod event;
mod input;
mod utils;

pub use self::input::Direction;
pub use self::utils::CameraRotation;

use specs::{self, Join};
use cgmath::{self, Quaternion, Rotation3, Rad};
use common;

use control;
use player::components::Player;
use math::functions;
use self::event::Event;

/// A type alias for convenience
type Euler = cgmath::Euler<Rad<::Float>>;

pub struct System {
    /// Receives events
    input: event::EventReceiver,
    /// The rotation to apply to the player entity
    rotate_direction: Option<CameraRotation>,
    /// Internally used for clamping the camera controls
    current_direction: Euler,
    /// Input state
    input_state: input::InputState,
}

impl System {
    pub fn new(input: event::EventReceiver) -> Self {
        Self {
            input: input,
            rotate_direction: None,
            current_direction: cgmath::Quaternion::from_angle_x(cgmath::Deg(0.0)).into(),
            input_state: Default::default(),
        }
    }

    fn check_input(&mut self) {
        while let Ok(e) = self.input.try_recv() {
            match e {
                Event::RotateCamera(rot) => self.rotate_direction = Some(rot),
                Event::EnableMoveDirection(direction) => {
                    self.input_state.insert(
                        input::InputState::from_bits(direction as _)
                            .unwrap(),
                    );
                }
                Event::DisableMoveDirection(direction) => {
                    self.input_state.remove(
                        input::InputState::from_bits(direction as _)
                            .unwrap(),
                    );
                }
            }
        }
    }

    /// Applies the provided rotation to the current direction, and returns the new value
    fn update_direction(&mut self, rot: CameraRotation) -> Quaternion<::Float> {
        let current = &mut self.current_direction;

        // The pitch, yaw, and roll values are stored internally
        // Rotations are added to the stored values, and the rotation is constructed each
        // update, instead of accumulating
        current.x = functions::clamp(current.x + rot.pitch(), Rad(0.0), Rad(3.14));
        current.y = functions::wrap(current.y + rot.yaw(), Rad(-3.14), Rad(3.14));

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
    direction: specs::WriteStorage<'a, common::Direction>,
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

            if let Some(angle) = self.input_state.get_movement_angle() {
                let dir = d.0 * Quaternion::from_angle_y(angle);
                c.move_in_direction(dir, 10.0);
            }
        }
    }
}
