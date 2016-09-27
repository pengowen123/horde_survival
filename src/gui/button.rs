use glutin::{Window, CursorState};
use collision::{Aabb, Aabb2};
use cgmath::Point2;

use hsgraphics::{GraphicsState, shapes2d};
use hsgraphics::object2d::Object2d;
use gamestate::GameState;
use gameloop::LoopType;
use utils::set_cursor_state;
use gui::{UIObject, UIState};
use gui::utils::get_button_color;
use consts::graphics::*;
use gui::consts::*;

pub struct Button {
    rect: Aabb2<f32>,
    id: u32,
    object: Object2d,
}

impl Button {
    // NOTE: rect coords should be in the range (-1.0, 1.0)
    pub fn new(id: u32, mut rect: Aabb2<f32>, graphics: &mut GraphicsState) -> Button {
        let object_rect = shapes2d::rectangle_from_aabb(&rect, get_button_color(id), graphics);
        let object = Object2d::from_slice(&mut graphics.factory, &object_rect, graphics.main_color.clone());

        rect.min.x *= graphics.pixel_size.0 * GUI_SCALE;
        rect.max.x *= graphics.pixel_size.0 * GUI_SCALE;
        rect.min.y *= graphics.pixel_size.1 * GUI_SCALE;
        rect.max.y *= graphics.pixel_size.1 * GUI_SCALE;

        Button {
            rect: rect,
            id: id,
            object: object,
        }
    }
}

impl UIObject for Button {
    fn draw(&self, graphics: &mut GraphicsState) {
        self.object.encode(&mut graphics.encoder, &graphics.pso2d);
    }

    fn is_selected(&self, point: Point2<f32>) -> bool {
         self.rect.contains(point)
    }

    fn select(&mut self,
              _: &mut Option<u32>,
              state: &mut UIState,
              game: &mut GameState,
              loop_type: &mut LoopType,
              window: &Window,
              graphics: &mut GraphicsState) {

        println!("Selected button with ID {}", self.id);
        match *state {
            UIState::MainMenu => {
                match self.id {
                    NEW_GAME => {
                        info!("Started new game");
                        game.new_game();
                        game.next_round();
                        set_cursor_state(window, CursorState::Hide);
                        *loop_type = LoopType::Game;
                    },
                    _ => crash!("Button not found: {}", self.id),
                }
            },
            UIState::ShopMenu => {
                match self.id {
                    START_ROUND => {
                        *loop_type = LoopType::Game;
                        set_cursor_state(window, CursorState::Hide);
                        game.next_round();
                        graphics.last_cursor_pos = graphics.window_center;
                    },
                    _ => crash!("Button not found: {}", self.id),
                }
            },
            _ => {},
        }
    }
}
