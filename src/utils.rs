use std::time::Duration;

pub fn millis(duration: Duration) -> u64 {
    let secs = duration.as_secs();
    let nanos = duration.subsec_nanos() as u64;

    secs * 1000 + nanos / 1_000_000
}
