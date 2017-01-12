//! Player state and player related functions

pub mod class;
pub mod input;
mod abilities;
mod inventory;

pub use self::class::Class;

use player::abilities::*;
use player::input::Input;
use world::Coords;
use hsgraphics::camera::Camera;
use items::*;
use entity::*;
use consts::*;

use std::collections::HashMap;

/// Player state
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
    pub camera: Camera,
}

pub enum AbilityID {
    A,
    B,
    C,
    D,
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
            input: Default::default(),
            camera: Camera { coords: coords, ..Default::default() },
        }
    }
}

// Misc
impl Player {
    /// Gives the player gold
    pub fn give_gold(&mut self, gold: usize) {
        self.gold += gold;
    }

    /// Updates the player's cooldowns
    pub fn update_cooldowns(&mut self) {
        for cooldown in &mut self.current_cooldowns {
            if *cooldown > 0 {
                *cooldown -= 1;
            }
        }
    }

    /// Starts a cooldown, used when an ability is cast
    pub fn start_cooldown(&mut self, id: usize) {
        assert!(id < 4, "Invalid cooldown ID");
        let base = match self.class {
            Class::Warrior => WARRIOR_COOLDOWNS[id],
        };

        self.current_cooldowns[id] = apply(&self.cooldown_mods, base as f64) as usize;
    }

    /// Resets controls to the default state
    pub fn reset_controls(&mut self) {
        self.input = Default::default();
        self.left_click = false;
    }
}

/// Returns the ability's function and animation info
fn get_ability_and_animation(class: Class,
                             ability: usize)
                             -> (fn(&mut Player, &mut Vec<Entity>), (usize, usize)) {
    match class {
        Class::Warrior => {
            match ability {
                0 => (warrior_ability_0, WARRIOR_ANIM_0),
                1 => (warrior_ability_1, WARRIOR_ANIM_1),
                2 => (warrior_ability_2, WARRIOR_ANIM_2),
                3 => (warrior_ability_3, WARRIOR_ANIM_3),
                _ => unreachable!(),
            }
        }
    }
}

// Abilities
impl Player {
    /// Casts the ability with the given ID
    pub fn cast_ability(&mut self, entities: &mut Vec<Entity>, ability: AbilityID) {
        let mut is_casting = false;
        // Get the index of the cooldown in the cooldowns array
        let cooldown_index = ability as usize;
        let ability_id = cooldown_index + 1;

        // Get ability and animation info
        let (ability, animation_info) = get_ability_and_animation(self.class, cooldown_index);

        // If the cooldown is 0 (can cast the ability), perform additional checks and start the
        // animation
        if self.current_cooldowns[cooldown_index] == 0 {
            let player_entity = entities.iter_mut()
                .find(|e| e.id == self.entity_id)
                .expect("Player entity not found");

            is_casting = player_entity.animations.is_playing(ability_id);

            // This is here rather than in the next block to avoid finding player_entity twice
            if player_entity.animations.is_finished(ability_id) && !is_casting {
                player_entity.animations.start(ability_id, animation_info.0, animation_info.1);
            }
        }

        if is_casting {
            // Start the cooldown, and cast the ability
            self.start_cooldown(cooldown_index);
            ability(self, entities);
        }
    }
}
