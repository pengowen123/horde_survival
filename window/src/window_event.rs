//! Window event processing
//!
//! Turns raw window events into a single, high level event type

use common::glutin::{self, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
use common::cgmath::{self, Rad};
use common::{self, UiState, shrev};
use slog;

use input::Direction;

/// A type alias for an event channel that uses `Event`
pub type EventChannel = shrev::EventChannel<Event>;
pub type ReaderId = shrev::ReaderId<Event>;

/// Camera sensitivity
const SENSITIVITY: ::Float = 0.0035;

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
    /// A movement key was pressed or released
    ChangeMovementKeyState(Direction, State),
    /// The camera has rotated
    RotateCamera(CameraRotation),
    /// The shaders should be reloaded
    ReloadShaders,
    /// The game was unpaused
    Unpaused,
    /// The window was resized
    WindowResized(glutin::dpi::LogicalSize),
}

/// Processes the provided window event, sending the processed version to the event channel
pub fn process_window_event(
    channel: &mut EventChannel,
    window: &glutin::Window,
    event: &WindowEvent,
) {
    match *event {
        WindowEvent::CursorMoved {
            device_id: _,
            modifiers: _,
            position,
        } => {
            common::utils::set_cursor_pos_to_window_center(window);

            let center = match common::utils::get_window_center(window) {
                Some(c) => c,
                None => return,
            };

            let diff_pitch = position.y  - center.y;
            let diff_yaw = position.x  - center.x;

            // Yaw control is inverted, so invert it again to fix it
            let diff_yaw = -diff_yaw;
            // Pitch control is also inverted
            let diff_pitch = -diff_pitch;
            
            let rot_pitch = diff_pitch as ::Float * SENSITIVITY;
            let rot_yaw = diff_yaw as ::Float * SENSITIVITY;
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

/// Like `process_window_event`, but deals solely with graphics-related events and is run even while
/// not in game
pub fn process_window_event_graphics(
    channel: &mut EventChannel,
    window: &glutin::Window,
    event: &WindowEvent,
    ui_state: &mut UiState,
    log: &slog::Logger,
) {
    match *event {
        WindowEvent::KeyboardInput {
            input: KeyboardInput {
                state,
                virtual_keycode,
                modifiers: _,
                ..
            },
            ..
        } => {
            if let Some(key) = virtual_keycode {
                match key {
                    VirtualKeyCode::Escape => {
                        if let ElementState::Pressed = state {
                            match *ui_state {
                                UiState::InGame => {
                                    window.hide_cursor(false);
                                    *ui_state = UiState::PauseMenu;
                                }
                                UiState::PauseMenu => {
                                    unpause(ui_state, window, channel);
                                }
                                _ => {},
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        WindowEvent::Resized(new_size) => {
            channel.single_write(Event::WindowResized(new_size));

            // Center the cursor so the camera doesn't jump when the window resizes
            common::utils::set_cursor_pos_to_window_center(window);

            let physical_size = new_size.to_physical(window.get_hidpi_factor());
            info!(log, "Window resized"; o!("new_dimensions" => format!("{:?}", physical_size)));
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


/// Unpauses the game
pub fn unpause(
    ui_state: &mut UiState,
    window: &glutin::Window,
    event_channel: &mut EventChannel,
) {
    // Center the cursor so the camera doesn't jump when the game unpauses
    // FIXME: This only mitigates the issue but doesn't fix it entirely
    common::utils::set_cursor_pos_to_window_center(window);

    window.hide_cursor(true);
    *ui_state = UiState::InGame;
    event_channel.single_write(Event::Unpaused);
}
