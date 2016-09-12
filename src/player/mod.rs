pub mod class;
mod abilities;
mod inventory;

pub use self::class::Class;

use self::abilities::*;

use winapi::POINT;

use items::*;
use entity::*;
use consts::*;

use std::collections::HashMap;

pub struct Player {
    // Player state
    pub gold: usize,
    pub class: Class,
    pub inventory: HashMap<usize, Item>,
    pub dead: bool,

    // Id numbers
    pub entity_id: usize,
    pub player_id: usize,

    // Cooldowns
    pub current_cooldowns: [usize; 4],
    pub cooldown_mods: Vec<Modifier>,

    // Controls
    pub left_click: bool,
    pub capture_cursor: bool,
    pub mouse: POINT,
    pub move_forward: bool,
    pub move_left: bool,
    pub move_backward: bool,
    pub move_right: bool,
}

// Constructor
impl Player {
    pub fn new(entity_id: usize, player_id: usize, class: Class) -> Player {
        Player {
            entity_id: entity_id,
            player_id: player_id,
            gold: 0,
            class: class,
            current_cooldowns: [0; 4],
            cooldown_mods: Vec::new(),
            inventory: base_inventory(),
            dead: false,
            left_click: false,
            capture_cursor: false,
            mouse: POINT { x: 0, y: 0 },
            move_forward: false,
            move_left: false,
            move_backward: false,
            move_right: false,
        }
    }
}

// Misc
impl Player {
    pub fn give_gold(&mut self, gold: usize) {
        self.gold += gold;
    }

    pub fn update_cooldowns(&mut self) {
        for cooldown in &mut self.current_cooldowns {
            if *cooldown > 0 {
                *cooldown -= 1;
            }
        }
    }
}

// Abilities
impl Player {
    pub fn ability_0(&mut self, entities: &mut Vec<Entity>) {
        self.current_cooldowns[0] = self.cooldown_mods.iter().fold(WARRIOR_COOLDOWN_0, |acc, x| (acc as f64 * x.value) as usize);

        match self.class {
            Class::Warrior => {
                warrior_ability_0(self, entities);
            },
        }
    }

    pub fn ability_1(&mut self, entities: &mut Vec<Entity>) {
        self.current_cooldowns[1] = self.cooldown_mods.iter().fold(WARRIOR_COOLDOWN_1, |acc, x| (acc as f64 * x.value) as usize);

        match self.class {
            Class::Warrior => {
                warrior_ability_1(self, entities);
            },
        }
    }

    pub fn ability_2(&mut self, entities: &mut Vec<Entity>) {
        self.current_cooldowns[2] = self.cooldown_mods.iter().fold(WARRIOR_COOLDOWN_2, |acc, x| (acc as f64 * x.value) as usize);

        match self.class {
            Class::Warrior => {
                warrior_ability_2(self, entities);
            },
        }
    }

    pub fn ability_3(&mut self, entities: &mut Vec<Entity>) {
        self.current_cooldowns[3] = self.cooldown_mods.iter().fold(WARRIOR_COOLDOWN_3, |acc, x| (acc as f64 * x.value) as usize);

        match self.class {
            Class::Warrior => {
                warrior_ability_3(self, entities);
            },
        }
    }
}
