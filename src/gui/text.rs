use glutin::Window;
use cgmath::Point2;
use collision::Aabb2;

use hsgraphics::GraphicsState;
use gamestate::GameState;
use gameloop::LoopType;
use gui::*;

pub struct Text {
    pos: [f32; 2],
    text: String,
    cache_id: usize,
}

impl Text {
    pub fn new(text: String, pos: [f32; 2]) -> Text {
        Text {
            pos: pos,
            text: text,
            cache_id: 0,
        }
    }

    pub fn new_on_button(text: String, rect: &Aabb2<f32>) -> Text {
        // TODO: Make the text centered on the button rect
        Text::new(text, [rect.min.x, rect.min.y])
    }
}

impl UIObject for Text {
    fn draw(&self, graphics: &mut GraphicsState) {
    }

    fn is_selected(&self, _: Point2<f32>) -> bool { false }
    fn select(&mut self, _: &mut Option<u32>, state: &UIState, _: &mut GameState, _: &mut LoopType, _: &Window, _: &mut GraphicsState) -> UIState { state.clone() }
}
