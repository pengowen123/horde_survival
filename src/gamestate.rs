use consts::*;
use player::*;
use rand::{Rng, thread_rng};
use entity::{Entity, EntityType};
use entity::flags::Team;
use map::Map;
use world::Coords;
use items::weapon::get_random_monster_weapon;
use hsgraphics::{GraphicsState, get_camera};

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
        let map = Map::new(0.0, Coords::origin(), (TEST_SPAWN_POINTS.0.to_vec(), TEST_SPAWN_POINTS.1.to_vec()));
        let player = Player::new(player_entity_id, 0, Class::Warrior, map.player_spawn.clone());
        let player_coords = map.player_spawn.clone();

        GameState {
            entities: vec![Entity::player(player_coords, player.entity_id, Team::Players)],
            next_entity_id: player.entity_id + 1,
            player: player,
            map: map,
            wave: 0,
            bounty: BASE_BOUNTY,
        }
    }

    pub fn new_game(&mut self) {
        info!("Started new game");
        *self = GameState::new();
        // TODO: Delete this
        self.entities[0].armor[0] = balance::items::armor::HEAL;
    }
}

// Rounds
impl GameState {
    pub fn end_round(&mut self, graphics: &mut GraphicsState) {
        let mut player = self.entities.iter().find(|e| e.id == self.player.entity_id).expect("Player entity disappeared").clone();

        player.health = player.max_hp;
        player.coords = self.map.player_spawn.clone();
        graphics.camera = get_camera(self.map.player_spawn.clone(), self.player.direction, graphics.aspect_ratio);
        self.player.coords = self.map.player_spawn.clone();
        self.player.current_cooldowns = [0; 4];

        self.entities.clear();
        self.entities.push(player);
        self.player.reset_controls();
    }

    pub fn next_round(&mut self) {
        self.wave += 1;
        self.bounty = BASE_BOUNTY + (BOUNTY_GROWTH * self.wave as f64) as usize;
        self.populate();

        info!("Starting wave {}", self.wave);
    }

    pub fn round_finished(&self) -> bool {
        self.entities.iter()
            .filter(|e| e.is_monster() && e.team == Team::Monsters)
            .next().is_none()
    }
}

// Entities
impl GameState {
    pub fn spawn_entity(&mut self, mut entity: Entity) {
        entity.id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(entity);
    }

    pub fn populate(&mut self) {
        let count = BASE_WAVE_SIZE + (WAVE_SIZE_GROWTH * self.wave);
        let mut rng = thread_rng();

        for _ in 0..count {
            let mut coords = self.map.choose_random_spawn_point();
            // NOTE: Spawn different monster types in the future

            coords.translate(&Coords::new((rng.gen::<f64>() - 0.5) * MONSTER_SPAWN_RADIUS,
                                          0.0,
                                          (rng.gen::<f64>() - 0.5) * MONSTER_SPAWN_RADIUS));

            let mut entity = Entity::monster(EntityType::Zombie, coords, 0, Team::Monsters, self.bounty);

            let weapon = get_random_monster_weapon(self.wave);

            entity.current_weapon = weapon;

            self.spawn_entity(entity);
        }
    }
}
