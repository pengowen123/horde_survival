//! Window event processing
//!
//! Turns raw window events into a single, high level event type

use common::glutin::{self, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
use common::cgmath::{self, Rad};
use common::shrev;

use input::Direction;

/// A type alias for an event channel that uses `Event`
pub type EventChannel = shrev::EventChannel<Event>;
pub type ReaderId = shrev::ReaderId<Event>;

/// A state flag used for state change events
pub enum State {
    Enabled,
    Disabled,
}

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

/// The event type that is sent through the event channel
pub enum Event {
    ChangeMovementKeyState(Direction, State),
    RotateCamera(CameraRotation),
    ReloadShaders,
}

/// Processes the provided window event, sending the processed version to the event channel
///
pub fn process_window_event(
    channel: &mut EventChannel,
    window: &glutin::Window,
    event: WindowEvent,
) {
    match event {
        WindowEvent::CursorMoved {
            device_id: _,
            modifiers: _,
            position: (x, y),
        } => {
            // TODO: Refactor this to make this code cleaner
            let (w, h) = match window.get_inner_size() {
                Some(s) => s,
                None => return,
            };
            let middle = (w as ::Float / 2.0, h as ::Float / 2.0);

            // TODO: Move this to somewhere that makes more sense, and remove the return type from
            //       this function
            window.set_cursor_position(middle.0 as i32, middle.1 as i32)
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
            let camera_rot = CameraRotation::new(cgmath::Rad(rot_pitch), cgmath::Rad(rot_yaw));

            channel.single_write(Event::RotateCamera(camera_rot));
        }
        WindowEvent::KeyboardInput {
            input: KeyboardInput {
                state,
                virtual_keycode,
                modifiers: _,
                ..
            },
            ..
        } => {
            let mut event = None;

            if let Some(key) = virtual_keycode {
                match key {
                    VirtualKeyCode::W => {
                        event = Some(get_movement_event(Direction::Forward, state))
                    }
                    VirtualKeyCode::A => {
                        event = Some(get_movement_event(Direction::Left, state))
                    }
                    VirtualKeyCode::S => {
                        event = Some(get_movement_event(Direction::Backward, state))
                    }
                    VirtualKeyCode::D => {
                        event = Some(get_movement_event(Direction::Right, state))
                    }
                    VirtualKeyCode::F1 => {
                        if let ElementState::Pressed = state {
                            event = Some(Event::ReloadShaders);
                        }
                    },
                    _ => {}
                }
            }

            if let Some(e) = event {
                channel.single_write(e);
            }
        }
        _ => {}
    }
}

/// Returns an event that enables or disables the provided movement direction, depending on whether
/// `state` is `ElementState::Pressed`
fn get_movement_event(direction: Direction, state: ElementState) -> Event {
    let state = match state {
        ElementState::Pressed => State::Enabled,
        ElementState::Released => State::Disabled,
    };

    Event::ChangeMovementKeyState(direction, state)
}
