//! Window event processing
//!
//! Processes raw window events into higher-level data types and sends them to the event handling
//! system to be handled

use glutin::{self, WindowEvent, KeyboardInput, VirtualKeyCode};
use cgmath;

use std::sync::mpsc;

use player::control;

/// A type for processing window events.
pub struct SenderHub {
    control: mpsc::Sender<control::Event>,
}

impl SenderHub {
    /// Returns a `SenderHub`, as well as its `ReceiverHub` that will receive events from the
    /// sender.
    pub fn new() -> (SenderHub, ReceiverHub) {
        let (snd, recv) = mpsc::channel();

        (SenderHub { control: snd }, ReceiverHub { control: recv })
    }

    /// Processes the provided window event, sending the process version to the event handler
    pub fn process_window_event(&self, window: &glutin::Window, event: WindowEvent) {
        match event {
            WindowEvent::MouseMoved {
                device_id: _,
                position: (x, y),
            } => {
                // TODO: Refactor this to make this code cleaner
                let (w, h) = match window.get_inner_size_pixels() {
                    Some(s) => s,
                    None => return,
                };
                let middle = (w as ::Float / 2.0, h as ::Float / 2.0);

                window
                    .set_cursor_position(middle.0 as i32, middle.1 as i32)
                    .expect("Failed to set cursor position");

                let diff_pitch = y as ::Float - middle.1;
                let diff_yaw = x as ::Float - middle.0;

                let sensitivity = 0.0035;

                // Yaw control is inverted, so invert it again to fix it
                let diff_yaw = -diff_yaw;
                // Pitch control is also inverted
                let diff_pitch = -diff_pitch;

                let rot_pitch = diff_pitch as ::Float * sensitivity;
                let rot_yaw = diff_yaw as ::Float * sensitivity;
                let camera_rot =
                    control::CameraRotation::new(cgmath::Rad(rot_pitch), cgmath::Rad(rot_yaw));

                self.control
                    .send(control::Event::RotateCamera(camera_rot))
                    .expect("Failed to send event");
            }
            WindowEvent::KeyboardInput {
                input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::W), .. }, ..
            } => {
                self.control.send(control::Event::MoveForward).expect(
                    "Failed to send event",
                );
            }
            _ => {}
        }
    }
}

pub struct ReceiverHub {
    control: control::EventReceiver,
}

impl ReceiverHub {
    pub fn into_receiver(self) -> control::EventReceiver {
        self.control
    }
}
