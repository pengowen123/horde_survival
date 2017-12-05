//! Utility functions and types for player controls

use cgmath::Rad;

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
