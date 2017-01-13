use consts::*;
use player::*;
use rand::{Rng, thread_rng};
use entity::{Entity, EntityType};
use entity::flags::Team;
use map::Map;
use world::Coords;
use items::weapon::get_random_monster_weapon;
use hsgraphics::{GraphicsState, initial_camera};

pub struct GameState {
    pub entities: Vec<Entity>,
    pub next_entity_id: usize,
    pub player: Player,
    pub map: Map,
    pub wave: usize,
    pub bounty: usize,
}

// Constructor
impl GameState {
    pub fn new() -> GameState {
        let player_entity_id = 0;

        // NOTE: This is a test map, remove this when real maps are implemented
        let map = Map::new(0.0, Default::default(), TEST_SPAWN_POINTS.to_vec());

        // Create the player
        let player = Player::new(player_entity_id, 0, Class::Warrior, map.player_spawn);
        let player_coords = map.player_spawn;

        GameState {
            entities: vec![Entity::player(player_coords, player.entity_id, Team::Players)],
            next_entity_id: player.entity_id + 1,
            player: player,
            map: map,
            wave: 0,
            bounty: BASE_BOUNTY,
        }
    }

    /// Resets the game to its default state
    pub fn new_game(&mut self) {
        info!("Started new game");
        *self = GameState::new();

        // NOTE: Delete this
        self.entities[0].armor[0] = balance::items::armor::HEAL;
        self.entities[0].current_weapon = balance::items::weapon::TEST_BOW;
    }
}

// Rounds
impl GameState {
    /// Used when a round ends, runs cleanup code
    pub fn end_round(&mut self, graphics: &mut GraphicsState) {
        let mut player = self.entities
            .iter()
            .find(|e| e.id == self.player.entity_id)
            .expect("Player entity disappeared")
            .clone();

        player.health = player.max_hp;
        player.coords = self.map.player_spawn;
        graphics.camera = initial_camera(self.map.player_spawn, graphics.aspect_ratio);
        self.player.camera.coords = self.map.player_spawn;
        self.player.current_cooldowns = [0; 4];

        self.entities.clear();
        self.entities.push(player);
        self.player.reset_controls();
    }

    /// Used when a round starts, updates game counters like wave number, and starts the round
    pub fn next_round(&mut self) {
        self.wave += 1;
        self.bounty = BASE_BOUNTY + (BOUNTY_GROWTH * self.wave as f64) as usize;
        self.populate();

        info!("Starting wave {}", self.wave);
    }

    /// Returns whether the found has finished
    pub fn is_round_finished(&self) -> bool {
        self.entities
            .iter()
            .find(|e| e.is_monster() && e.team == Team::Monsters)
            .is_none()
    }
}

// Entities
impl GameState {
    /// Spawns an entity
    pub fn spawn_entity(&mut self, mut entity: Entity) {
        entity.id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(entity);
    }

    /// Spawns enemies, used when the wave starts
    pub fn populate(&mut self) {
        let count = BASE_WAVE_SIZE + (WAVE_SIZE_GROWTH * self.wave);
        let mut rng = thread_rng();

        for _ in 0..count {
            // TODO: Use a real spawn location algorithm
            let mut coords = self.map.choose_random_spawn_point();
            // TODO: Spawn different monster types in the future

            coords.translate(&Coords::new((rng.gen::<f64>() - 0.5) * MONSTER_SPAWN_RADIUS,
                                          0.0,
                                          (rng.gen::<f64>() - 0.5) * MONSTER_SPAWN_RADIUS));

            let mut entity =
                Entity::monster(EntityType::Zombie, coords, 0, Team::Monsters, self.bounty);

            let weapon = get_random_monster_weapon(self.wave);

            entity.current_weapon = weapon;

            self.spawn_entity(entity);
        }
    }
}
