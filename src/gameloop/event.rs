use glutin::{self, Window, Event};

use gamestate::GameState;
use hsgraphics::GraphicsState;
use hscontrols::handle_keyboard_input;

pub fn handle_event(event: Event, game: &mut GameState, graphics: &mut GraphicsState, window: &Window) {
    match event {
        Event::Resized(..) => {
            window.set_inner_size(graphics.window_size.0, graphics.window_size.1);
        },
        Event::MouseMoved(x, y) => {
            graphics.last_cursor_pos = (x, y);
        },
        Event::KeyboardInput(state, scan_code, key) => {
            let player = &mut game.player;

            handle_keyboard_input(key,
                                  state,
                                  scan_code,
                                  &mut game.entities,
                                  player);
        },
        Event::MouseInput(state, button) => {
            if let glutin::MouseButton::Left = button {
                    game.player.left_click = state == glutin::ElementState::Pressed;
            }
        },
        Event::Closed => graphics.should_close = true,
        _ => {},
    }
}
