#[allow(match_same_arms)]
pub fn get_movement_offset(forward: bool, left: bool, right: bool, backward: bool) -> f64 {
    match (forward, left, right, backward) {
        // 1 direction
        (true, false, false, false) => 0.0,
        (false, false, true, false) => 270.0,
        (false, true, false, false) => 90.0,
        (false, false, false, true) => 180.0,

        // 2 directions
        (true, false, true, false) => 315.0,
        (true, true, false, false) => 45.0,
        (false, false, true, true) => 225.0,
        (false, true, false, true) => 135.0,
        (true, false, false, true) => 0.0,
        (false, true, true, false) => 90.0,

        // 3 directions
        (false, true, true, true) => 180.0,
        (true, true, false, true) => 90.0,
        (true, false, true, true) => 270.0,
        (true, true, true, false) => 0.0,

        // 4 directions
        (true, true, true, true) => 0.0,

        // 0 directions
        (false, false, false, false) => unreachable!(),
    }
}
