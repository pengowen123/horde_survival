use glutin::{self, Window, Event};

use gamestate::GameState;
use hsgraphics::GraphicsState;
use hscontrols::handle_keyboard_input;

pub fn handle_event(event: Event, game: &mut GameState, graphics: &mut GraphicsState, window: &Window) -> bool {
    match event {
        Event::Resized(w, h) => {
            graphics.resize(w, h, &window);
            false
        },
        Event::MouseMoved(x, y) => {
            graphics.last_cursor_pos = (x, y);
            false
        },
        Event::KeyboardInput(state, scan_code, key) => {
            let player = &mut game.player;

            handle_keyboard_input(key,
                                  state,
                                  scan_code,
                                  &mut game.entities,
                                  player,
                                  &window);

            false
        },
        Event::MouseInput(state, button) => {
            match button {
                glutin::MouseButton::Left => {
                    game.player.left_click = state == glutin::ElementState::Pressed;
                },
                _ => {},
            }

            false
        },
        Event::Closed => {
            return true;
        }
        _ => false,
    }
}
