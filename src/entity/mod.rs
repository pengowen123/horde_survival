pub mod modifiers;
pub mod entity_type;
pub mod attack;

pub use self::entity_type::EntityType;
pub use self::modifiers::Modifier;
pub use self::attack::try_attack;

use items::*;
use world::*;
use consts::balance::*;

#[derive(Clone)]
pub struct Entity {
    pub id: usize,
    pub entity_type: EntityType,
    pub coords: Coords,
    pub direction: (f64, f64),
    pub health: f64,
    pub attack_animation: usize,
    pub damage_mods: Vec<Modifier>,
    pub as_mods: Vec<Modifier>,
    pub damage_taken_mods: Vec<Modifier>,
    pub movespeed_mods: Vec<Modifier>,
    pub is_enemy: bool,
    pub dummy: bool,
    pub current_weapon: Weapon,
    pub lifetime: usize,
}

impl Entity {
    pub fn new(id: usize,
               health: f64,
               coords: Coords,
               entity_type: EntityType,
               is_enemy: bool,
               dummy: bool,
               direction: (f64, f64),
               lifetime: usize) -> Entity {

        Entity {
            id: id,
            entity_type: entity_type,
            coords: coords,
            direction: direction,
            health: health,
            attack_animation: 0,
            damage_mods: Vec::new(),
            as_mods: Vec::new(),
            damage_taken_mods: Vec::new(),
            movespeed_mods: Vec::new(),
            is_enemy: is_enemy,
            dummy: dummy,
            current_weapon: WEAPON_UNARMED,
            lifetime: lifetime,
        }
    }
}

impl Entity {
    pub fn damage(&mut self, damage: f64) -> bool {
        self.health -= self.damage_taken_mods.iter().fold(damage, |acc, x| acc * x.value);
        self.is_dead()
    }

    pub fn attack_entity(&self, other: &mut Entity) -> bool {
        let mut damage = self.current_weapon.damage;

        damage = self.damage_mods.iter().fold(damage, |acc, x| acc * x.value);

        other.damage(damage)
    }

    pub fn move_forward(&mut self, movement_offset: f64) {
        let speed = self.movespeed_mods.iter().fold(BASE_MOVESPEED, |acc, x| acc * x.value);

        self.coords.move_forward(self.direction.1 + movement_offset, speed);
    }

    pub fn set_direction(&mut self, direction: (f64, f64)) {
        self.direction = direction;
    }
}

impl Entity {
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }
}

