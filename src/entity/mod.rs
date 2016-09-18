pub mod modifiers;
pub mod entity_type;
pub mod attack;
pub mod update;
pub mod filter;
pub mod flags;

pub use self::entity_type::EntityType;
pub use self::modifiers::Modifier;
pub use self::attack::try_attack;
pub use self::update::*;
pub use self::filter::*;
pub use self::flags::*;

use items::*;
use world::*;
use player::Player;
use consts::balance::*;

#[derive(Clone)]
pub struct Entity {
    // Unique ID number
    pub id: usize,

    // Health
    pub health: f64,
    pub max_hp: f64,

    // Type of entity
    pub entity_type: EntityType,
    
    // World info
    pub coords: Coords,
    pub direction: (f64, f64),
    pub velocity: Velocity,

    // Modifiers
    pub damage_mods: Vec<Modifier>,
    pub as_mods: Vec<Modifier>,
    pub damage_taken_mods: Vec<Modifier>,
    pub movespeed_mods: Vec<Modifier>,

    // Physics info
    pub has_gravity: HasGravity,
    pub on_ground: bool,
    pub entity_height: f64,
    
    // Flags
    pub has_ai: HasAI,
    pub team: Team,
    pub is_dummy: IsDummy,

    // Inventory
    pub armor: [Armor; 4],
    pub current_weapon: Weapon,

    // Animations
    pub attack_animation: usize,

    // Misc
    pub lifetime: usize,
    pub spawned_by: Option<usize>,
    pub bounty: usize,

    // AI
    // TODO: Move these fields to an AI field of type AIData
    pub ai_projectile_error: f64,
    pub ai_target_id: usize,
    pub ai_consecutive_error_increases: usize,
}

// Constructor
impl Entity {
    pub fn new(id: usize,
               health: f64,
               max_hp: f64,
               mut coords: Coords,
               entity_type: EntityType,
               team: Team,
               is_dummy: IsDummy,
               direction: (f64, f64),
               lifetime: usize,
               bounty: usize,
               has_gravity: HasGravity,
               has_ai: HasAI,
               spawned_by: Option<usize>) -> Entity {

        let height = get_entity_height(&entity_type);
        coords.y += height;

        let mut movespeed_mods = Vec::new();
        let movespeed = get_movespeed(&entity_type);

        if let Some(m) = movespeed {
            movespeed_mods.push(Modifier::new(m, 0));
        }

        Entity {
            id: id,
            entity_height: height,
            coords: coords,
            entity_type: entity_type,
            direction: direction,
            health: health,
            max_hp: max_hp,
            attack_animation: 0,
            damage_mods: Vec::new(),
            as_mods: Vec::new(),
            damage_taken_mods: Vec::new(),
            movespeed_mods: movespeed_mods,
            team: team,
            is_dummy: is_dummy,
            lifetime: lifetime,
            velocity: Velocity::zero(),
            has_gravity: has_gravity,
            on_ground: false,
            has_ai: has_ai,
            current_weapon: UNARMED,
            armor: [HEAD_NONE, BODY_NONE, LEGS_NONE, FEET_NONE],
            spawned_by: spawned_by,
            ai_projectile_error: 0.0,
            ai_target_id: 0,
            ai_consecutive_error_increases: 0,
            bounty: bounty,
        }
    }
}

// Other constructors
impl Entity {
    pub fn player(coords: Coords, entity_id: usize, team: Team) -> Entity {
        Entity::new(entity_id,
                    PLAYER_HEALTH,
                    PLAYER_HEALTH,
                    coords,
                    EntityType::Player,
                    team,
                    IsDummy::False,
                    DEFAULT_DIRECTION,
                    INFINITE_LIFETIME,
                    0,
                    HasGravity::True,
                    HasAI::False,
                    None)
    }

    pub fn zombie(coords: Coords, entity_id: usize, team: Team, bounty: usize) -> Entity {
        Entity::new(entity_id,
                    ZOMBIE_HEALTH,
                    ZOMBIE_HEALTH,
                    coords,
                    EntityType::Zombie,
                    team,
                    IsDummy::False,
                    DEFAULT_DIRECTION,
                    INFINITE_LIFETIME,
                    bounty,
                    HasGravity::True,
                    HasAI::True,
                    None)
    }
}

// Misc
impl Entity {
    #[allow(dead_code)]
    pub fn heal(&mut self, amount: f64) {
        self.health += amount;

        if self.health > self.max_hp {
            self.health = self.max_hp;
        }
    }

    pub fn damage(&mut self, mut damage: f64, self_index: usize, hit_by: usize, entities: &mut Vec<Entity>, player: &mut Player) -> bool {
        for armor in &mut self.armor {
            if let Some(f) = armor.when_hit {
                f(self_index, hit_by, entities, player);
                damage *= armor.multiplier;
            }
        }

        self.health -= self.damage_taken_mods.iter().fold(damage, |acc, x| acc * x.value);
        self.is_dead()
    }

    pub fn get_damage(&self) -> f64 {
        self.damage_mods.iter().fold(self.current_weapon.damage, |acc, x| acc * x.value)
    }

    pub fn move_forward(&mut self, movement_offset: f64) {
        let speed = self.movespeed_mods.iter().fold(BASE_MOVESPEED, |acc, x| acc * x.value);

        self.coords.move_forward(self.direction.1 + movement_offset, speed);
    }

    pub fn has_gravity(&self) -> bool {
        self.has_gravity == HasGravity::True
    }

    pub fn kill(&mut self) {
        self.health = DEAD_ENTITY_HEALTH;
    }
}

// Flag test methods
impl Entity {
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    pub fn is_enemy_of(&self, other: &Entity) -> bool {
        self.team != other.team
    }

    pub fn is_dummy(&self) -> bool {
        self.is_dummy == IsDummy::True
    }

    pub fn has_ai(&self) -> bool {
        self.has_ai == HasAI::True
    }
}
