//! Item effects

//#![allow(dead_code, unused_variables)]

pub mod armor;
pub mod weapon;

use entity::Entity;
use player::Player;

/// A function that represents a weapon's special effect
///
/// The first argument is the index of the entity that hit the target
/// The second argument is the index of the entity that is being hit
/// The third argument is a list of all entities
/// The fourth argument is the player
pub type WeaponEffect = &'static Fn(usize, usize, &mut Vec<Entity>, &mut Player);

/// A function that represents an armor's special effect
///
/// The first argument is the index of the entity that hit the target
/// The second argument is the index of the entity that is being hit
/// The third argument is a list of all entities
/// The fourth argument is the player
pub type ArmorEffect = &'static Fn(usize, usize, &mut Vec<Entity>, &mut Player);
