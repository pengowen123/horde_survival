use winapi::{RECT, HWND};
use user32::GetWindowRect;

use consts::balance::*;
use entity::Modifier;

use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

pub const TPS: u64 = 30;

pub const fn time(seconds: f64) -> usize {
    (seconds * TPS as f64) as usize
}

pub fn update_modifiers(modifiers: &mut Vec<Modifier>) {
    *modifiers = modifiers.iter().cloned().filter(|m| !m.is_expired()).collect();

    for modifier in modifiers {
        modifier.update();
    }
}

pub fn get_bounty(wave: usize) -> usize {
    let wave = wave as f64;
    let bounty = BASE_BOUNTY as f64;

    (bounty + wave * BOUNTY_GROWTH) as usize
}

pub fn get_movement_offset(forward: bool, left: bool, right: bool, backward: bool) -> f64 {
    match (forward, left, right, backward) {
        // 1 direction
        (true, false, false, false) => 0.0,
        (false, true, false, false) => 270.0,
        (false, false, true, false) => 90.0,
        (false, false, false, true) => 180.0,

        // 2 directions
        (true, true, false, false)  => 315.0,
        (true, false, true, false)  => 45.0,
        (false, true, false, true)  => 225.0,
        (false, false, true, true)  => 135.0,
        (true, false, false, true)  => 0.0,
        (false, true, true, false)  => 90.0,

        // 3 directions
        (false, true, true, true)   => 180.0,
        (true, false, true, true)   => 90.0,
        (true, true, false, true)   => 270.0,
        (true, true, true, false)   => 0.0,
        
        // 4 directions
        (true, true, true, true)  => 0.0,
        
        // 0 directions
        (false, false, false, false) => unreachable!(),
    }
}

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
