use glutin::*;

use gamestate::GameState;
use hsgraphics::GraphicsState;
use gameloop::LoopType;
use gui::{UI, UIState};
use consts::graphics::GUI_CLEAR_COLOR;
use tps::Ticks;
use consts::misc::GUI_MAX_FPS;
use utils::*;

use std::time::Duration;
use std::thread;

pub fn run_gui(event: Option<Event>,
               ui: &mut UI,
               game: &mut GameState,
               graphics: &mut GraphicsState,
               window: &Window,
               ticks: &mut Ticks,
               loop_type: &mut LoopType) {

    ticks.begin_tick();

    let expected_elapsed = Duration::from_millis(1_000_000_000 / GUI_MAX_FPS / 1_000_000);
    ticks.set_expected_elapsed(expected_elapsed);

    if let Some(e) = event {
        match e {
            Event::Resized(..) => {
                // FIXME: This causes memory leaks, at least on Windows
                window.set_inner_size(graphics.window_size.0, graphics.window_size.1);
            },
            Event::MouseMoved(x, y) => {
                graphics.last_cursor_pos = (x, y);
            },
            Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
            },
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                match ui.state {
                    UIState::EscapeMenu => {
                        *loop_type = LoopType::Game;
                        set_cursor_state(window, CursorState::Hide);
                        graphics.reset_cursor(window);
                    },
                    UIState::OptionsMenu => {
                    },
                    UIState::MainMenu => {
                        graphics.should_close = true;
                    },
                    _ => {},
                }
            },
            Event::Closed => graphics.should_close = true,
            _ => {},
        }
    }
    
    if ticks.is_sleeping() {
        return;
    }

    graphics.update_dpi(window);
    graphics.encoder.clear(&graphics.data.out_color, GUI_CLEAR_COLOR);

    ticks.measure_frame_1();

    if graphics.options.display_debug {
        let frame = millis(ticks.get_debug_info()[0]);
        let string = format!("Horde Survival - frame {} ms | updates 0 ms | total {} ms", frame, frame);
        window.set_title(&string);
    }

    ticks.end_tick();
}
