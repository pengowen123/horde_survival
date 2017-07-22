//! A high-level event type

use std::sync::mpsc;

use super::utils::CameraRotation;
use super::input;

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
