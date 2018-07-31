//! Window event processing
//!
//! Turns raw window events into a single, high level event type

use common::glutin::{self, WindowEvent, KeyboardInput, VirtualKeyCode, ElementState};
use common::cgmath::{self, Rad};
use common::{self, UiState, shrev, config};
use slog;

use input::Direction;

/// A type alias for an event channel that uses `Event`
pub type EventChannel = shrev::EventChannel<Event>;
pub type ReaderId = shrev::ReaderId<Event>;

/// A state flag used for state change events
#[derive(Debug)]
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

/// The part of the configuration that has changed
#[derive(Debug)]
pub enum ChangedConfig {
    /// The graphics configuration has changed
    Graphics,
    /// The window configuration has changed
    ///
    /// Window size changes here do not actually resize the window. It should be resized as a result
    /// of this event happening, which will cause a `WindowResized` event to be sent.
    Window,
    /// The camera configuration has changed
    Camera,
    /// The key binding configuration has changed
    Bindings,
}

/// The event type that is sent through the event channel
#[derive(Debug)]
pub enum Event {
    /// A movement key was pressed or released, and the movement key state should be updated
    ChangeMovementKeyState(Direction, State),
    /// The camera should be rotated
    RotateCamera(CameraRotation),
    /// The shaders should be reloaded
    ReloadShaders,
    /// The game was unpaused
    Unpaused,
    /// The window was resized
    WindowResized(glutin::dpi::LogicalSize),
    /// The configuration has changed
    ///
    /// Handlers of this event must read the new configuration manually
    ConfigChanged(ChangedConfig),
}

/// Processes the provided window event, sending the processed version to the event channel
pub fn process_window_event(
    config: &config::Config,
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
            
            // TODO: Investigate how sensitivity scales with monitor size, and maybe fix the
            //       handling of it
            let rot_pitch = diff_pitch as ::Float * config.camera.sensitivity;
            let rot_yaw = diff_yaw as ::Float * config.camera.sensitivity;
            let camera_rot = CameraRotation::new(cgmath::Rad(rot_pitch), cgmath::Rad(rot_yaw));

            channel.single_write(Event::RotateCamera(camera_rot));
        }
        WindowEvent::KeyboardInput {
            input: KeyboardInput {
                state,
                virtual_keycode,
                modifiers,
                ..
            },
            ..
        } => {
            let mut event = None;

            let virtual_keycode = match virtual_keycode {
                Some(c) => c,
                // Do nothing if there is no virtual keycode (such as when Ctrl+Shift+Alt is
                // pressed)
                None => return,
            };
            let key: config::Key = virtual_keycode.into();

            let current_bind = config::Bind::new(key.clone(), modifiers.into());

            // Handle movement keys
            event = event.or_else(|| get_movement_event(&current_bind,
                                                        &config.bindings.move_forward,
                                                        state,
                                                        Direction::Forward));

            event = event.or_else(|| get_movement_event(&current_bind,
                                                        &config.bindings.move_backward,
                                                        state,
                                                        Direction::Backward));

            event = event.or_else(|| get_movement_event(&current_bind,
                                                        &config.bindings.move_left,
                                                        state,
                                                        Direction::Left));

            event = event.or_else(|| get_movement_event(&current_bind,
                                                        &config.bindings.move_right,
                                                        state,
                                                        Direction::Right));


            match state {
                ElementState::Pressed => {
                    if current_bind == config.bindings.reload_shaders {
                        event = Some(Event::ReloadShaders);
                    }
                }
                ElementState::Released => {
                }
            }

            if let Some(e) = event {
                channel.single_write(e);
            }
        }
        _ => {}
    }
}

/// Returns an event that enables or disables the provided movement direction, based on the provided
/// binding comparison and element state
fn get_movement_event(
    // The current binding based on the latest window event
    current_bind: &config::Bind,
    // The binding to compare to
    test_bind: &config::Bind,
    state: ElementState,
    direction: Direction,
) -> Option<Event> {
    match state {
        ElementState::Pressed => {
            if current_bind.key == test_bind.key {
                // While holding a movement key down, if the modifiers no longer match, disable
                // that movement direction
                let state = if current_bind.modifiers == test_bind.modifiers {
                    State::Enabled
                } else {
                    State::Disabled
                };
                Some(Event::ChangeMovementKeyState(direction, state))
            } else {
                None
            }
        }
        ElementState::Released => {
            // If a movement key is released, disable that movement direction regardless of modifier
            // state
            if current_bind.key == test_bind.key {
                Some(Event::ChangeMovementKeyState(direction, State::Disabled))
            } else {
                None
            }
        }
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
