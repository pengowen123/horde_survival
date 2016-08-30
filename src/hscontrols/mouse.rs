use consts::graphics::*;
use user32::{GetCursorPos, SetCursorPos};
use utils::*;
use winapi::*;

pub fn center_mouse(mouse: &mut POINT,
                    hwnd: *mut HWND__,
                    window_position: &mut (i32, i32),
                    center: &mut (i32, i32)) {

    unsafe {
        GetCursorPos(mouse);
    }

    *window_position = get_window_position(hwnd, *window_position);

    *center = (window_position.0 + WINDOW_WIDTH as i32 / 2,
              window_position.1 + WINDOW_HEIGHT as i32 / 2);

    mouse.x -= center.0;
    mouse.y -= center.1;

    unsafe {
        SetCursorPos(center.0, center.1);
    }
}
