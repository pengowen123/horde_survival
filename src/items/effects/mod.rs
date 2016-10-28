// NOTE: To have an effect kill an entity without first checking entity type, set the entity's
//       health to a negative value rather than settings its lifetime to 1, as the latter may kill the
//       player entity

#![allow(dead_code, unused_variables)]

pub mod armor;
pub mod weapon;

use entity::Entity;
use player::Player;

pub type ItemEffect = &'static Fn(usize, usize, &mut Vec<Entity>, &mut Player);
