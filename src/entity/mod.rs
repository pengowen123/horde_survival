//! Everything related to entities

#[macro_use]
pub mod modifiers;
pub mod entity_type;
pub mod attack;
pub mod update;
#[macro_use]
pub mod filter;
pub mod flags;
pub mod animation;

pub use self::entity_type::EntityType;
pub use self::modifiers::*;
pub use self::attack::try_attack;
pub use self::update::*;
pub use self::filter::*;
pub use self::flags::*;
pub use self::animation::*;

use collision::Aabb3;

use consts::balance::*;
use items::*;
use world::*;
use player::Player;

/// The hitbox of an entity
pub type Hitbox = Aabb3<f64>;

/// An entity
#[derive(Clone)]
// TODO: Add a gold field, and make everything use that instead to avoid having to return gold
//       gained
//       Set player.gold to the player entity's gold value each tick
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
    pub hitbox: Hitbox,

    // Flags
    pub has_ai: HasAI,
    pub team: Team,
    pub is_dummy: IsDummy,

    // Inventory
    pub armor: [Armor; 4],
    pub current_weapon: Weapon,

    // Animations
    pub animations: AnimationList,

    // Misc
    // NOTE: To kill an entity without first checking entity type, set the entity's health to a
    //       negative value rather than settings its lifetime to 1, as the latter may kill the
    //       player entity
    pub lifetime: usize,
    pub spawned_by: Option<usize>,
    pub bounty: usize,
    pub needs_update: bool,
    pub attack: bool,

    // AI
    // TODO: Move these fields to an AI field of type AIData
    pub ai_projectile_error: f64,
    pub ai_target_id: usize,
    // Counter for how many times the AI's projectile error has increased
    pub ai_consecutive_error_increases: usize,
}

// Constructor
impl Entity {
    #[allow(too_many_arguments)]
    pub fn new(id: usize,
               health: f64,
               max_hp: f64,
               coords: Coords,
               entity_type: EntityType,
               team: Team,
               is_dummy: IsDummy,
               direction: (f64, f64),
               lifetime: usize,
               bounty: usize,
               has_gravity: HasGravity,
               has_ai: HasAI,
               spawned_by: Option<usize>)
               -> Entity {

        /// Get the hitbox of the entity's type
        let hitbox = get_hitbox(&entity_type, &coords);

        /// Multiply the base entity movespeed by the entity's type's base movespeed
        let mut movespeed_mods = Vec::new();
        get_movespeed(&entity_type).map(|m| movespeed_mods.push(modifier!(multiplicative, m, 0)));

        Entity {
            id: id,
            coords: coords,
            hitbox: hitbox,
            entity_type: entity_type,
            direction: direction,
            health: health,
            max_hp: max_hp,
            animations: Default::default(),
            damage_mods: Vec::new(),
            as_mods: Vec::new(),
            damage_taken_mods: Vec::new(),
            movespeed_mods: movespeed_mods,
            team: team,
            is_dummy: is_dummy,
            lifetime: lifetime,
            velocity: Default::default(),
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
            needs_update: true,
            attack: false,
        }
    }
}

// Other constructors
impl Entity {
    /// Convenience constructor for creating player entities
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

    /// Convenience constructor for creating monsters such as zombies
    pub fn monster(entity_type: EntityType,
                   coords: Coords,
                   entity_id: usize,
                   team: Team,
                   bounty: usize)
                   -> Entity {

        let health = get_monster_health(&entity_type);

        Entity::new(entity_id,
                    health,
                    health,
                    coords,
                    entity_type,
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
    /// Heals the entity by the given amount, up to its maximum health
    #[allow(dead_code)]
    pub fn heal(&mut self, amount: f64) {
        self.health += amount;

        if self.health > self.max_hp {
            self.health = self.max_hp;
        }
    }

    /// Damages the entity, with damage received modifiers applied
    /// Returns whether the entity has died due to the damage
    pub fn damage(&mut self,
                  mut damage: f64,
                  self_index: usize,
                  hit_by: usize,
                  entities: &mut Vec<Entity>,
                  player: &mut Player)
                  -> bool {
        for armor in &mut self.armor {
            if let Some(f) = armor.when_hit {
                f(hit_by, self_index, entities, player);
                damage *= armor.multiplier;
            }
        }

        self.health -= apply(&self.damage_taken_mods, damage);
        self.is_dead()
    }

    /// Returns the damage of the entity's weapon, with damage modifiers applied
    pub fn get_damage(&self) -> f64 {
        apply(&self.damage_mods, self.current_weapon.damage)
    }

    /// Moves the entity forward, in it's direction plus the given offset angle
    /// For example, if an entity's direction is 90, and the offset is 50, it moves in the
    /// direction 140
    pub fn move_forward(&mut self, movement_offset: f64) {
        let speed = apply(&self.movespeed_mods, BASE_MOVESPEED);

        self.coords.move_forward(self.direction.1 + movement_offset, speed);
    }

    /// Kills the entity
    pub fn kill(&mut self) {
        self.health = DEAD_ENTITY_HEALTH;
    }

    /// Updates the entity's hitbox to its current position
    pub fn update_hitbox(&mut self) {
        self.hitbox = get_hitbox(&self.entity_type, &self.coords);
    }

    /// Returns the height of the entity's hitbox
    pub fn get_height(&self) -> f64 {
        self.hitbox.max.y - self.hitbox.min.y
    }

    /// Returns whether the entity is of a monster type (e.g. a Zombie)
    pub fn is_monster(&self) -> bool {
        // TODO: Update this when new entity types are added
        match self.entity_type {
            EntityType::Zombie => true,
            _ => false,
        }
    }
}

// Flag test methods
impl Entity {
    /// Returns whether the entity is dead
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    /// Returns whether the given entity is an enemy of this one
    pub fn is_enemy_of(&self, other: &Entity) -> bool {
        self.team != other.team
    }

    /// Returns whether the entity is a dummy
    pub fn is_dummy(&self) -> bool {
        self.is_dummy == IsDummy::True
    }

    /// Returns whether the entity is AI controlled
    pub fn has_ai(&self) -> bool {
        self.has_ai == HasAI::True
    }

    /// Returns whether the entity is affected by gravity
    pub fn has_gravity(&self) -> bool {
        self.has_gravity == HasGravity::True
    }
}
