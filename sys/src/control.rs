//! Controls system to let players control their entity

use specs::{self, Join, DispatcherBuilder};
use cgmath::{self, Quaternion, Rotation3};

use std::sync::mpsc;

use world;
use player;
use window::event;

/// An event sent by a player, for example when a player presses a key an event will be generated
pub enum Event {
    /// A player rotated the camera (the direction will be added to the player entity's direction)
    RotateCamera(CameraRotation),
}

#[derive(Clone, Copy, Debug)]
pub struct CameraRotation {
    pitch: cgmath::Rad<::Float>,
    yaw: cgmath::Rad<::Float>,
}

impl CameraRotation {
    pub fn new<T: Into<cgmath::Rad<::Float>>>(pitch: T, yaw: T) -> Self {
        Self {
            pitch: pitch.into(),
            yaw: yaw.into(),
        }
    }
}

pub type EventReceiver = mpsc::Receiver<Event>;

pub struct System {
    input: EventReceiver,
    rotate_direction: Option<CameraRotation>,
}

impl System {
    pub fn new(input: EventReceiver) -> Self {
        Self {
            input: input,
            rotate_direction: None,
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
    player: specs::ReadStorage<'a, player::Player>,
    direction: specs::WriteStorage<'a, world::components::Direction>,
}

impl<'a> specs::System<'a> for System {
    type SystemData = Data<'a>;

    fn run(&mut self, mut data: Self::SystemData) {
        // TODO: Maybe use delta time here for controls
        self.check_input();

        for (d, _) in (&mut data.direction, &data.player).join() {
            if let Some(rot) = self.rotate_direction.clone() {
                // Create a quaternion from the direction
                let rot_pitch = Quaternion::from_angle_x(rot.pitch);
                let rot_yaw = Quaternion::from_angle_y(rot.yaw);
                let camera_direction = rot_pitch * rot_yaw;

                d.0 = d.0 * camera_direction;
            }
        }
    }
}

/// Initializes controls-related components and systems
pub fn init<'a, 'b>(
    dispatcher: DispatcherBuilder<'a, 'b>,
) -> (DispatcherBuilder<'a, 'b>, event::SenderHub) {

    let (snd, recv) = event::SenderHub::new();
    let control = System::new(recv.into_receiver());
    let dispatcher = dispatcher.add(control, "control", &[]);

    (dispatcher, snd)
}
