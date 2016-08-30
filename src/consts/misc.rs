pub const TPS: u64 = 30;

pub const fn time(seconds: f64) -> usize { (seconds * TPS as f64) as usize }
