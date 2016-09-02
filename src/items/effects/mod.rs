#![allow(dead_code, unused_variables)]

pub mod armor;
pub mod weapon;

use entity::Entity;
use player::Player;

pub type ItemEffect = &'static Fn(usize, usize, &mut Vec<Entity>, &mut Player);
