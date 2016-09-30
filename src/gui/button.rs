use glutin::{Window, CursorState};
use collision::{Aabb, Aabb2};
use cgmath::Point2;

use hsgraphics::{GraphicsState, shapes2d};
use hsgraphics::object2d::Object2d;
use gamestate::GameState;
use gameloop::LoopType;
use utils::set_cursor_state;
use gui::*;

pub struct Button {
    rect: Aabb2<f32>,
    id: u32,
    object: Object2d,
}

impl Button {
    pub fn new(id: u32, mut rect: Aabb2<f32>, graphics: &mut GraphicsState, state: UIState, align: Align) -> Button {
        align.apply(&mut rect);

        let texture = graphics.get_texture(get_button_texture_id(state, id));
        let texture = graphics.get_texture(13);
        let object_rect = shapes2d::rectangle_from_aabb(&rect);
        let object = Object2d::from_slice(&mut graphics.factory, &object_rect, texture);

        Button {
            rect: rect,
            id: id,
            object: object,
        }
    }
}

impl UIObject for Button {
    fn draw(&self, graphics: &mut GraphicsState) {
        self.object.encode(&mut graphics.encoder, &graphics.pso2d, &mut graphics.data2d);
    }

    fn is_selected(&self, point: Point2<f32>) -> bool {
         self.rect.contains(point)
    }

    fn select(&mut self,
              _: &mut Option<u32>,
              state: &UIState,
              game: &mut GameState,
              loop_type: &mut LoopType,
              window: &Window,
              graphics: &mut GraphicsState) -> UIState {

        println!("Selected button with ID {}", self.id);
        match *state {
            UIState::MainMenu => {
                match self.id {
                    NEW_GAME => {
                        game.new_game();
                        game.next_round();
                        set_cursor_state(window, CursorState::Hide);
                        graphics.reset_cursor(window);
                        *loop_type = LoopType::Game;
                    },
                    GOTO_OPTIONS => {
                        return UIState::OptionsMenu;
                    },
                    EXIT_GAME_MAIN => {
                        graphics.should_close = true;
                    },
                    _ => crash!("Button not found: {}", self.id),
                }
            },
            UIState::OptionsMenu => {
                match self.id {
                    EXIT_OPTIONS => {
                        return UIState::MainMenu;
                    },
                    _ => crash!("Button not found: {}", self.id),
                }
            }
            UIState::ShopMenu => {
                match self.id {
                    START_ROUND => {
                        *loop_type = LoopType::Game;
                        set_cursor_state(window, CursorState::Hide);
                        graphics.reset_cursor(window);
                        game.next_round();
                    },
                    _ => crash!("Button not found: {}", self.id),
                }
            },
            UIState::EscapeMenu => {
                match self.id {
                    RETURN_TO_GAME => {
                        *loop_type = LoopType::Game;
                        set_cursor_state(window, CursorState::Hide);
                        graphics.reset_cursor(window);
                    },
                    EXIT_GAME_ESC => {
                        return UIState::MainMenu;
                    },
                    _ => crash!("Button not found: {}", self.id),
                }
            },
        }

        state.clone()
    }
}
