pub mod modifiers;
pub mod entity_type;

pub use self::entity_type::EntityType;
pub use self::modifiers::Modifier;

use world::*;
use consts::balance::*;

pub struct Entity {
    pub id: usize,
    pub entity_type: EntityType,
    pub coords: Coords,
    pub direction: (f64, f64),
    pub health: f64,
    pub damage_mods: Vec<Modifier>,
    pub as_mods: Vec<Modifier>,
    pub damage_taken_mods: Vec<Modifier>,
    pub movespeed_mods: Vec<Modifier>,
    pub is_enemy: bool,
}

impl Entity {
    pub fn new(id: usize,
               health: f64,
               coords: Coords,
               entity_type: EntityType,
               is_enemy: bool) -> Entity {

        Entity {
            id: id,
            entity_type: entity_type,
            coords: coords,
            direction: (90.0, 0.0),
            health: health,
            damage_mods: Vec::new(),
            as_mods: Vec::new(),
            damage_taken_mods: Vec::new(),
            movespeed_mods: Vec::new(),
            is_enemy: is_enemy,
        }
    }
}

impl Entity {
    pub fn damage(&mut self, damage: f64) {
        self.health -= self.damage_taken_mods.iter().fold(damage, |acc, x| acc * x.value);
    }

    pub fn attack_entity(&self, other: &mut Entity) {
        let mut damage = 1.0 * BASE_DAMAGE;

        damage = self.damage_mods.iter().fold(damage, |acc, x| acc * x.value);

        other.damage(damage);
    }

    pub fn try_attack(&self, entities: &mut [Entity], enemy: bool) -> bool {
        let mut success = false;

        for point in self.coords.ray(ATTACK_RADIUS / 2.0) {
            // NOTE: check that entity type is not for dummies
            match entities.iter_mut().filter(|e| {
                e.coords.in_radius(&self.coords, ATTACK_RADIUS) && e.is_enemy == enemy
            }).nth(0) {
                Some(e) => {
                    self.attack_entity(e);
                    success = true;
                },
                None => success = false,
            }

            break;
        }

        success
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
