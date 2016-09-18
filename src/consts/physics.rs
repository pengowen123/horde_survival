use consts::misc::TPS;

// NOTE: Be careful when changing this, as the AI's ranged weapon control reacts to changes
pub const G: f64 = 3.0;
pub const GRAVITY: f64 = G / TPS as f64;
