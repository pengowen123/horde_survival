use glutin::{Window, CursorState};
use collision::{Aabb, Aabb2};
use cgmath::Point2;

use hsgraphics::{GraphicsState, shapes2d};
use hsgraphics::object2d::Object2d;
use gamestate::GameState;
use gameloop::LoopType;
use utils::set_cursor_state;
use hslog::CanUnwrap;
use gui::*;

pub struct Button {
    rect: Aabb2<f32>,
    id: u32,
    object: Object2d,
}

impl Button {
    pub fn new(id: u32, mut rect: Aabb2<f32>, graphics: &mut GraphicsState, state: UIState, align: Align) -> (Button, Aabb2<f32>) {
        align.apply(&mut rect);

        let name = get_button_texture_name(state, id);
        let texture = unwrap_or_log!(graphics.assets.get_or_load_texture(&name, &mut graphics.factory),
                                     "Failed to load texture: {}", name);
        let object_rect = shapes2d::rectangle_from_aabb(&rect);
        let object = Object2d::from_slice(&mut graphics.factory, &object_rect, texture.clone());

        let button = Button {
            rect: rect,
            id: id,
            object: object,
        };

        (button, rect)
    }
}

impl UIObject for Button {
    fn draw(&self, graphics: &mut GraphicsState) {
        self.object.encode(&mut graphics.encoder, &graphics.pso2d, &mut graphics.data2d);
    }

    fn is_selected(&self, point: Point2<f32>) -> bool {
         self.rect.contains(point)
    }

    fn get_layer(&self) -> usize { 1 }

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
            UIState::LoadingScreen => {},
        }

        state.clone()
    }
}
