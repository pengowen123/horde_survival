pub mod class;
pub mod input;
mod abilities;
mod inventory;

pub use self::class::Class;

use player::abilities::*;
use player::input::Input;
use world::Coords;
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

    // Input
    pub input: Input,
    pub left_click: bool,
    pub mouse: (i32, i32),

    // Used for camera control
    pub direction: (f64, f64),
    pub coords: Coords,
}

// Constructor
impl Player {
    pub fn new(entity_id: usize, player_id: usize, class: Class, coords: Coords) -> Player {
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
            mouse: (0, 0),
            input: Input::new(),
            direction: START_CAMERA_ANGLE,
            coords: coords,
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

    pub fn start_cooldown(&mut self, id: usize) {
        let base = match self.class {
            Class::Warrior => WARRIOR_COOLDOWNS[id],
        };

        self.current_cooldowns[id] = apply(&self.cooldown_mods, base as f64) as usize;
    }

    pub fn reset_controls(&mut self) {
        self.input = Input::new();
        self.left_click = false;
    }
}

// Abilities
impl Player {
    pub fn ability_0(&mut self, entities: &mut Vec<Entity>) {
        let mut is_casting = false;

        if self.current_cooldowns[0] == 0 {
            let player_entity = entities.iter_mut()
                .find(|e| e.id == self.entity_id)
                .expect("Player entity not found");

            is_casting = player_entity.animations.is_casting(1);

            if player_entity.animations.can_cast(1) && !is_casting {
                player_entity.animations.start(1, WARRIOR_PRE_0, WARRIOR_POST_0);
            }
        }

        if is_casting {
            self.start_cooldown(0);

            match self.class {
                Class::Warrior => {
                    warrior_ability_0(self, entities);
                }
            }
        }
    }

    pub fn ability_1(&mut self, entities: &mut Vec<Entity>) {
        let mut is_casting = false;

        if self.current_cooldowns[1] == 0 {
            let player_entity = entities.iter_mut()
                .find(|e| e.id == self.entity_id)
                .expect("Player entity not found");

            is_casting = player_entity.animations.is_casting(2);

            if player_entity.animations.can_cast(2) && !is_casting {
                player_entity.animations.start(2, WARRIOR_PRE_1, WARRIOR_POST_1);
            }
        }

        if is_casting {
            self.start_cooldown(1);

            match self.class {
                Class::Warrior => {
                    warrior_ability_1(self, entities);
                }
            }
        }
    }

    pub fn ability_2(&mut self, entities: &mut Vec<Entity>) {
        let mut is_casting = false;

        if self.current_cooldowns[2] == 0 {
            let player_entity = entities.iter_mut()
                .find(|e| e.id == self.entity_id)
                .expect("Player entity not found");

            is_casting = player_entity.animations.is_casting(3);

            if player_entity.animations.can_cast(3) && !is_casting {
                player_entity.animations.start(3, WARRIOR_PRE_2, WARRIOR_POST_2);
            }
        }

        if is_casting {
            self.start_cooldown(2);

            match self.class {
                Class::Warrior => {
                    warrior_ability_2(self, entities);
                }
            }
        }
    }

    pub fn ability_3(&mut self, entities: &mut Vec<Entity>) {
        let mut is_casting = false;

        if self.current_cooldowns[3] == 0 {
            let player_entity = entities.iter_mut()
                .find(|e| e.id == self.entity_id)
                .expect("Player entity not found");

            is_casting = player_entity.animations.is_casting(4);

            if player_entity.animations.can_cast(4) && !is_casting {
                player_entity.animations.start(4, WARRIOR_PRE_3, WARRIOR_POST_3);
            }
        }

        if is_casting {
            self.start_cooldown(3);

            match self.class {
                Class::Warrior => {
                    warrior_ability_3(self, entities);
                }
            }
        }
    }
}
