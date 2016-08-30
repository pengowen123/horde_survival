pub const BASE_BOUNTY: usize = 10;
pub const BOUNTY_GROWTH: f64 = 0.4;

pub fn get_bounty(wave: usize) -> usize {
    let wave = wave as f64;
    let bounty = BASE_BOUNTY as f64;

    (bounty + wave * BOUNTY_GROWTH) as usize
}
