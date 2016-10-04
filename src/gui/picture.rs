use glutin::Window;
use collision::Aabb2;
use cgmath::Point2;

use gui::*;
use hsgraphics::*;
use hsgraphics::object2d::Object2d;
use gamestate::GameState;
use gameloop::LoopType;

pub struct Picture {
    object: Object2d,
    layer: usize,
}

impl Picture {
    pub fn new(mut rect: Aabb2<f32>, graphics: &mut GraphicsState, texture: Texture, align: Align, layer: usize) -> Picture {
        align.apply(&mut rect);

        let object_rect = shapes2d::rectangle_from_aabb(&rect);
        let object = Object2d::from_slice(&mut graphics.factory, &object_rect, texture);

        Picture {
            object: object,
            layer: layer,
        }
    }
}

impl UIObject for Picture {
    fn draw(&self, graphics: &mut GraphicsState) { self.object.encode(&mut graphics.encoder, &graphics.pso2d, &mut graphics.data2d); }
    fn is_selected(&self, _: Point2<f32>) -> bool { false }
    fn select(&mut self, _: &mut Option<u32>, state: &UIState, _: &mut GameState, _: &mut LoopType, _: &Window, _: &mut GraphicsState) -> UIState { state.clone() }
    fn get_layer(&self) -> usize { self.layer }
}
