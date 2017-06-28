//! Window event processing
//! Processes window events and sends them to the event system to be handled

use glutin::{self, WindowEvent};
use cgmath;

use std::sync::mpsc;

use control;
use math::direction;

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

    /// Processes the
    pub fn process_window_event(&self, window: &glutin::Window, event: WindowEvent) {
        match event {
            WindowEvent::MouseMoved(x, y) => {
                let (w, h) = match window.get_inner_size_pixels() {
                    Some(s) => s,
                    None => return,
                };
                let middle = (w as i32 / 2, h as i32 / 2);

                window
                    .set_cursor_position(middle.0, middle.1)
                    .expect("Failed to set cursor position");

                let diff_pitch = y - middle.1;
                let diff_yaw = x - middle.0;

                let sensitivity = 0.0035;

                // Yaw control is inverted, so invert it again to fix it
                let diff_yaw = -diff_yaw;

                let direction =
                    direction::Direction::new(cgmath::Rad(diff_pitch as ::Float * sensitivity),
                                              cgmath::Rad(diff_yaw as ::Float * sensitivity));

                self.control
                    .send(control::Event::RotateCamera(direction))
                    .expect("Failed to send event");
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
