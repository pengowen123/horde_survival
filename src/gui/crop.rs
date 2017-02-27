//! Functions for cropping conrod primitives to scizzor rectangles

use conrod::Rect;
use conrod::text::rt;

use std::ops::{Sub, Div, Mul};

use super::utils::rt_rect_from_corners;

/// A convenience wrapper around the `crop_dimension` function
macro_rules! crop_dimension {
    ($dim:expr, $a:expr, $op:tt, $b:expr,
     $c:ident,
     $uv:ident,
     $side_length:expr,
     $uv_side_length:expr,
     $uv_op:tt
     ) => {{
        crop_dimension(
            $a[$dim],
            $b[$dim],
            &mut $c[$dim],
            &mut $uv[$dim],
            $side_length,
            $uv_side_length,
            |a, b| a $op b,
            |a, b| a $uv_op b
        );
    }}
}

/// Crops a `Rect` to the `scizzor` rectangle
/// Returns the new `Rect`
pub fn rect(rect: Rect, scizzor: Rect) -> Rect {
    // Don't crop if unnecessary
    if rect == scizzor {
        return rect;
    }

    // The four corners of the rectangle
    let mut bl = rect.bottom_left();
    let br = rect.bottom_right();
    let tl = rect.top_left();
    let mut tr = rect.top_right();

    // The four corners of the scizzor rectangle
    let bl_scizzor = scizzor.bottom_left();
    let br_scizzor = scizzor.bottom_right();
    let tl_scizzor = scizzor.top_left();
    let tr_scizzor = scizzor.top_right();

    // Clamp bottom left corner
    if bl[0] < bl_scizzor[0] {
        bl[0] = bl_scizzor[0];
    }
    if bl[1] < bl_scizzor[1] {
        bl[1] = bl_scizzor[1];
    }

    // Clamp bottom right corner
    if br[0] > br_scizzor[0] {
        tr[0] = br_scizzor[0];
    }
    if br[1] < br_scizzor[1] {
        bl[1] = br_scizzor[1];
    }

    // Clamp top left corner
    if tl[0] < tl_scizzor[0] {
        bl[0] = tl_scizzor[0];
    }
    if tl[1] > tl_scizzor[1] {
        tr[1] = tl_scizzor[1];
    }

    // Clamp top right corner
    if tr[0] > tr_scizzor[0] {
        tr[0] = tr_scizzor[0];
    }
    if tr[1] > tr_scizzor[1] {
        tr[1] = tr_scizzor[1];
    }

    Rect::from_corners(bl, tr)
}

/// Crops an image's `Rect` and its UV `Rect` to the `scizzor` rectangle
/// Returns the new image and UV `Rect`s
pub fn image(rect: Rect, uv_rect: Rect, scizzor: Rect) -> (Rect, Rect) {
    if rect == scizzor {
        return (rect, uv_rect);
    }

    // The four corners of the rectangle
    let mut bl = rect.bottom_left();
    let br = rect.bottom_right();
    let tl = rect.top_left();
    let mut tr = rect.top_right();

    // The bottom left and top right corners of the UV rectangle
    let mut uv_bl = uv_rect.bottom_left();
    let mut uv_tr = uv_rect.top_right();

    // The four corners of the scizzor rectangle
    let bl_scizzor = scizzor.bottom_left();
    let br_scizzor = scizzor.bottom_right();
    let tl_scizzor = scizzor.top_left();
    let tr_scizzor = scizzor.top_right();

    // The side lengths of the rectangle
    let x_len = tr[0] - bl[0];
    let y_len = tr[1] - bl[1];

    // The side lengths of the UV rectangle
    let uv_x_len = uv_tr[0] - uv_bl[0];
    let uv_y_len = uv_tr[1] - uv_bl[1];

    // Clamp bottom left corner
    crop_dimension!(0, bl, <, bl_scizzor, bl, uv_tr, x_len, uv_x_len, +);
    crop_dimension!(1, bl, <, bl_scizzor, bl, uv_tr, y_len, uv_y_len, -);

    // Clamp bottom right corner
    crop_dimension!(0, br, >, br_scizzor, tr, uv_bl, x_len, uv_x_len, -);
    crop_dimension!(1, br, <, br_scizzor, bl, uv_tr, y_len, uv_y_len, -);

    // Clamp top left corner
    crop_dimension!(0, tl, <, tl_scizzor, bl, uv_tr, x_len, uv_x_len, +);
    crop_dimension!(1, tl, >, tl_scizzor, tr, uv_bl, y_len, uv_y_len, +);

    // Clamp top right corner
    crop_dimension!(0, tr, >, tr_scizzor, tr, uv_bl, x_len, uv_x_len, -);
    crop_dimension!(1, tr, >, tr_scizzor, tr, uv_bl, y_len, uv_y_len, +);

    (Rect::from_corners(bl, tr), Rect::from_corners(uv_bl, uv_tr))
}

/// Crops a glyph's rectangle and UV rectangle to the scizzor `Rect`
/// The given rectangles are from `rusttype`, not `conrod`
/// Returns the new UV and glyph rectangles
pub fn text(uv_rect: rt::Rect<f32>,
            rect: rt::Rect<f32>,
            scizzor: Rect)
            -> (rt::Rect<f32>, rt::Rect<f32>) {
    let min = rect.min;
    let max = rect.max;
    let uv_min = uv_rect.min;
    let uv_max = uv_rect.max;

    // The four corners of the rectangle
    let mut bl = [min.x as f64, min.y as f64];
    let br = [max.x as f64, min.y as f64];
    let tl = [min.x as f64, max.y as f64];
    let mut tr = [max.x as f64, max.y as f64];

    // The bottom left and top right corners of the UV rectangle
    let mut uv_bl = [uv_min.x as f64, uv_min.y as f64];
    let mut uv_tr = [uv_max.x as f64, uv_max.y as f64];

    // The four corners of the scizzor rectangle
    let bl_scizzor = scizzor.bottom_left();
    let br_scizzor = scizzor.bottom_right();
    let tl_scizzor = scizzor.top_left();
    let tr_scizzor = scizzor.top_right();

    // The side lengths of the rectangle
    let x_len = (tr[0] - bl[0]) as f64;
    let y_len = (tr[1] - bl[1]) as f64;

    // The side lengths of the UV rectangle
    let uv_x_len = (uv_tr[0] - uv_bl[0]) as f64;
    let uv_y_len = (uv_tr[1] - uv_bl[1]) as f64;

    // Clamp bottom left corner
    crop_dimension!(0, bl, <, bl_scizzor, bl, uv_tr, x_len, uv_x_len, +);
    crop_dimension!(1, bl, <, bl_scizzor, bl, uv_tr, y_len, uv_y_len, -);

    // Clamp bottom right corner
    crop_dimension!(0, br, >, br_scizzor, tr, uv_bl, x_len, uv_x_len, -);
    crop_dimension!(1, br, <, br_scizzor, bl, uv_tr, y_len, uv_y_len, -);

    // Clamp top left corner
    crop_dimension!(0, tl, <, tl_scizzor, bl, uv_tr, x_len, uv_x_len, +);
    crop_dimension!(1, tl, >, tl_scizzor, tr, uv_bl, y_len, uv_y_len, +);

    // Clamp top right corner
    crop_dimension!(0, tr, >, tr_scizzor, tr, uv_bl, x_len, uv_x_len, -);
    crop_dimension!(1, tr, >, tr_scizzor, tr, uv_bl, y_len, uv_y_len, +);

    let bl = [bl[0] as f32, bl[1] as f32];
    let tr = [tr[0] as f32, tr[1] as f32];

    let uv_bl = [uv_bl[0] as f32, uv_bl[1] as f32];
    let uv_tr = [uv_tr[0] as f32, uv_tr[1] as f32];

    (rt_rect_from_corners(uv_bl, uv_tr), rt_rect_from_corners(bl, tr))
}

// Utility functions

/// Sets `crop_a` to `b` if `compare(a, b)`. Also adjusts `crop_uv` proportionally.
/// This means that if `a - b` is half of `len`, `crop_uv` will be adjusted by half of `uv_len`.
fn crop_dimension<N, C, O>(a: N,
                           b: N,
                           crop_a: &mut N,
                           crop_uv: &mut N,
                           len: N,
                           uv_len: N,
                           compare: C,
                           op: O)
    where C: Fn(N, N) -> bool,
          O: Fn(N, N) -> N,
          N: Copy + Div<Output = N> + Sub<Output = N> + Mul<Output = N> + Abs
{
    if compare(a, b) {
        let diff = (*crop_a - b).abs();
        // The ratio between how far `b` is from `crop_a`, and `len`
        let ratio = diff / len;
        // How far to move `crop_uv`
        let uv_diff = ratio * uv_len;

        // Apply the crop
        *crop_a = b;
        *crop_uv = op(*crop_uv, uv_diff);
    }

}

/// A trait for any type that has an absolute value
trait Abs {
    fn abs(self) -> Self;
}

impl Abs for i32 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Abs for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Abs for f32 {
    fn abs(self) -> Self {
        self.abs()
    }
}
