use glutin::{Window, Event, ElementState, MouseButton, VirtualKeyCode, CursorState};

use gamestate::GameState;
use hsgraphics::GraphicsState;
use hscontrols::handle_keyboard_input;
use gui::{UI, UIState};
use gameloop::LoopType;
use utils::set_cursor_state;

/// Handles all events from the window when in the Game mode
pub fn handle_event_game(event: Event,
                         game: &mut GameState,
                         graphics: &mut GraphicsState,
                         window: &Window) {

    // See glutin::Event docs
    match event {
        Event::Resized(..) => {
            // FIXME: Causes memory leaks, at least on Windows
            window.set_inner_size(graphics.window_size.0, graphics.window_size.1);
        }
        Event::MouseMoved(x, y) => {
            graphics.last_cursor_pos = (x, y);
        }
        Event::KeyboardInput(state, scan_code, key) => {
            let player = &mut game.player;

            handle_keyboard_input(key, state, scan_code, &mut game.entities, player);
        }
        Event::MouseInput(state, button) => {
            if let MouseButton::Left = button {
                game.player.left_click = state == ElementState::Pressed;
            }
        }
        Event::Closed => graphics.should_close = true,
        _ => {}
    }
}

/// Handles all events from the window when in the GUI mode
pub fn handle_event_gui(event: Event,
                        ui: &mut UI,
                        graphics: &mut GraphicsState,
                        window: &Window,
                        loop_type: &mut LoopType) {

    match event {
        Event::Resized(..) => {
            // FIXME: This causes memory leaks, at least on Windows
            window.set_inner_size(graphics.window_size.0, graphics.window_size.1);
        }
        Event::MouseMoved(x, y) => {
            graphics.last_cursor_pos = (x, y);
        }
        Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
            match ui.state {
                UIState::Pause => {
                    *loop_type = LoopType::Game;
                    set_cursor_state(window, CursorState::Hide);
                    graphics.reset_cursor(window);
                }
                UIState::Options => {
                    ui.state = UIState::Main;
                }
                UIState::Main => {
                    graphics.should_close = true;
                }
                _ => {}
            }
        }
        Event::Closed => graphics.should_close = true,
        _ => {}
    }
}
