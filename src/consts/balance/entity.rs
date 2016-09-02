use consts::time;

pub const BASE_MOVESPEED: f64 = 0.05;

pub const AI_RANGE_THRESHOLD: f64 = 0.75;

pub const GLOBAL_ATTACK_TIME: f64 = 1.0;

pub const MELEE_LINE_RADIUS: f64 = 0.3;
pub const MELEE_LINE_INTERVAL: f64 = MELEE_LINE_RADIUS / 2.0;

pub const RANGED_LINEAR_LIFETIME: usize = time(1.5);
pub const RANGED_ARC_LIFETIME: usize = time(4.0);
pub const RANGED_ARC_SPEED: f64 = 0.05;

pub const RANGED_RADIUS: f64 = 0.3;
pub const RANGED_INTERVAL: f64 = RANGED_RADIUS / 2.0;

pub const PROJECTILE_SPAWN_OFFSET: f64 = 0.1;
