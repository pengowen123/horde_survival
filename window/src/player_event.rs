//! A high-level event type to apply to the player entity

use common::cgmath::Rad;

use std::sync::mpsc;

use input;

/// An event that represents an action a player can take
pub enum Event {
    /// A player rotated the camera (the direction will be added to the player entity's direction)
    RotateCamera(CameraRotation),
    /// Enables the flag that represents whether the entity is moving in the provided direction
    EnableMoveDirection(input::Direction),
    /// Disables the flag that represents whether the entity is moving in the provided direction
    DisableMoveDirection(input::Direction),
}

/// A receiver for `Event`s
pub type EventReceiver = mpsc::Receiver<Event>;

/// A rotation to apply to the camera
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

    /// Returns the pitch of this rotation
    pub fn pitch(&self) -> Rad<::Float> {
        self.pitch
    }

    /// Returns the yaw of this rotation
    pub fn yaw(&self) -> Rad<::Float> {
        self.yaw
    }
}
