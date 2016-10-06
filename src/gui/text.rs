use glutin::Window;
use cgmath::Point2;
use collision::{Aabb, Aabb2};

use hsgraphics::*;
use gamestate::GameState;
use gameloop::LoopType;
use consts::text::BUTTON_TEXT_HEIGHT;
use gui::*;

pub struct Text {
    object: object2d::Object2d,
}

impl Text {
    pub fn new<S: Into<String>>(text: S, rect: Aabb2<f32>, graphics: &mut GraphicsState) -> Text {
        let text = text.into();
        let height = rect.dim().y * graphics.window_size.1 as f32 * BUTTON_TEXT_HEIGHT;
        let texture = graphics.create_text_texture(&text, height);
        let shape = shapes2d::rectangle_from_aabb(&rect);
        let object = object2d::Object2d::from_slice(&mut graphics.factory, &shape, texture);

        object.encode(&mut graphics.encoder, &graphics.pso2d, &mut graphics.data2d);
        Text {
            object: object,
        }
    }

    pub fn new_on_button<S: Into<String>>(text: S, rect: &Aabb2<f32>, graphics: &mut GraphicsState) -> Text {
        // TODO: Resize the rect to avoid stretching or compressing of the text
        Text::new(text, rect.clone(), graphics)
    }

    pub fn new_aligned<S: Into<String>>(text: S, mut rect: Aabb2<f32>, align: Align, graphics: &mut GraphicsState) -> Text {
        align.apply(&mut rect);

        Text::new(text, rect, graphics)
    }
}

impl UIObject for Text {
    fn draw(&self, graphics: &mut GraphicsState) {
        self.object.encode(&mut graphics.encoder, &mut graphics.pso2d, &mut graphics.data2d);
    }

    fn is_selected(&self, _: Point2<f32>) -> bool { false }
    fn select(&mut self,
              _: &mut Option<u32>,
              state: &UIState,
              _: &mut GameState,
              _: &mut LoopType,
              _: &Window,
              _: &mut GraphicsState) -> UIState { state.clone() }

    fn get_layer(&self) -> usize { 2 }
}
