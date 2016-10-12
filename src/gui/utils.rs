use collision::Aabb2;
use cgmath::Point2;

use gui::UIObject;
use hsgraphics::GraphicsState;

pub fn mouse_pos_as_point(state: &GraphicsState, pos: (i32, i32)) -> Option<Point2<f32>> {
    if (pos.0 < 0 || pos.0 > state.window_size.0 as i32) ||
       (pos.1 < 0 || pos.1 > state.window_size.1 as i32) {
        return None;
    }

    // NOTE: This point is rotated 90 degrees clockwise
    let mut point = Point2::new(pos.0 as f32 * state.pixel_size.0 - 0.5,
                                -(pos.1 as f32 * state.pixel_size.1 - 0.5));

    point *= 2.0;

    Some(point)
}

pub fn rect(a: (f32, f32), b: (f32, f32)) -> Aabb2<f32> {
    Aabb2::new(Point2::new(a.0, a.1), Point2::new(b.0, b.1))
}

pub fn uiobject<T: UIObject + 'static>(object: T) -> Box<UIObject> {
    Box::new(object)
}
