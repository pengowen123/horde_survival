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
pub const RANGED_ARC_SPEED: f64 = 0.05;
pub const RANGED_RADIUS: f64 = 0.5;
pub const RANGED_INTERVAL: f64 = RANGED_RADIUS / 2.0;
pub const PROJECTILE_SPAWN_OFFSET: f64 = 0.1;

// Type-specific entity consts
pub const PLAYER_HEALTH: f64 = 100.0;
pub const ZOMBIE_HEALTH: f64 = 100.0;

pub fn get_entity_height(entity_type: &EntityType) -> f64 {
    match *entity_type {
        EntityType::Player => 0.8,
        EntityType::Zombie => 0.8,
        _ => 0.0,
    }
}
