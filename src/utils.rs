use winapi::{RECT, HWND};
use user32::GetWindowRect;

use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

pub fn convert_str(string: &str) -> Vec<u16> {
    OsStr::new(string).encode_wide().chain(once(0)).collect()
}
pub fn get_window_position(hwnd: HWND, previous_pos: (i32, i32)) -> (i32, i32) {
    let mut rect = RECT { left: 0, top: 0, right: 0, bottom: 0};

    unsafe {
        if GetWindowRect(hwnd, &mut rect) == 0 {
            return previous_pos;
        }
    }

    (rect.left, rect.top)
}
