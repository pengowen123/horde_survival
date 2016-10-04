use glutin::*;

use gui::mouse_pos_as_point;
use gamestate::GameState;
use hsgraphics::GraphicsState;
use gameloop::LoopType;
use gui::{UI, UIState};
use consts::graphics::GUI_CLEAR_COLOR;
use utils::*;
use tps::Ticks;

pub fn run_gui(event: Option<Event>,
               ui: &mut UI,
               game: &mut GameState,
               graphics: &mut GraphicsState,
               window: &Window,
               ticks: &mut Ticks,
               loop_type: &mut LoopType) {

    ticks.begin_tick();

    if let Some(e) = event {
        match e {
            Event::Resized(..) => {
                window.set_inner_size(graphics.window_size.0, graphics.window_size.1);
            },
            Event::MouseMoved(x, y) => {
                graphics.last_cursor_pos = (x, y);
            },
            Event::MouseInput(ElementState::Pressed, MouseButton::Left) => {
                let mouse = mouse_pos_as_point(&*graphics, graphics.last_cursor_pos);
                mouse.map_or((), |m| ui.click(m, game, loop_type, window, graphics));
            },
            Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => {
                match ui.state {
                    UIState::ShopMenu => {
                    },
                    UIState::EscapeMenu => {
                        *loop_type = LoopType::Game;
                        set_cursor_state(window, CursorState::Hide);
                        graphics.reset_cursor(window);
                    },
                    UIState::OptionsMenu => {
                        ui.set_state(UIState::MainMenu, graphics);
                    },
                    UIState::MainMenu => {
                        graphics.should_close = true;
                    },
                    UIState::LoadingScreen => {},
                }
            },
            Event::Closed => graphics.should_close = true,
            _ => {},
        }
    }
    
    graphics.encoder.clear(&graphics.data.out_color, GUI_CLEAR_COLOR);
    ui.draw(graphics);
    graphics.draw_gui(window);

    if graphics.options.display_debug {
        ticks.measure_frame_1();
        let frame = millis(ticks.get_debug_info()[0]);
        let string = format!("Horde Survival - frame {} ms | updates 0 ms | total {} ms", frame, frame);
        window.set_title(&string);
    }
}
