use consts::time;
use entity::EntityType;

// Special values
pub const DEAD_ENTITY_HEALTH: f64 = -1_000_000.0;
pub const INFINITE_LIFETIME: usize = 0;

// Base entity consts
pub const BASE_MOVESPEED: f64 = 0.05;
pub const GLOBAL_ATTACK_TIME: f64 = 1.0;
pub const DEFAULT_DIRECTION: (f64, f64) = (90.0, 0.0);

// Attacks
pub const MELEE_LINE_RADIUS: f64 = 0.3;
pub const MELEE_LINE_INTERVAL: f64 = MELEE_LINE_RADIUS / 2.0;
pub const RANGED_LINEAR_LIFETIME: usize = time(1.5);
pub const RANGED_LINEAR_SPEED: f64 = 0.5;
pub const RANGED_ARC_SPEED: f64 = 0.2;
pub const RANGED_RADIUS: f64 = 0.175;
pub const RANGED_INTERVAL: f64 = RANGED_RADIUS / 2.0;
pub const PROJECTILE_SPAWN_OFFSET: f64 = -0.1;

// Type-specific entity consts
pub const PLAYER_HEALTH: f64 = 100.0;

// Misc
pub const MONSTER_SPAWN_RADIUS: f64 = 2.5;

pub fn get_movespeed(entity_type: &EntityType) -> Option<f64> {
    match *entity_type {
        EntityType::Player => Some(1.5),
        EntityType::Zombie => Some(0.75),
        _ => None,
    }
}

pub fn get_monster_health(entity_type: &EntityType) -> f64 {
    match *entity_type {
        EntityType::Zombie => 100.0,
        _ => 0.0,
    }
}
