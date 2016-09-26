use rand::{Rng, thread_rng};
use consts::*;
use entity::flags::Team;
use entity::{Entity, EntityType};
use player::*;
use map::Map;
use world::Coords;
use items::weapon::get_random_monster_weapon;

pub struct GameState {
    pub entities: Vec<Entity>,
    pub next_entity_id: usize,
    pub player: Player,
    pub map: Map,
    pub wave: usize,
    pub bounty: usize,
}

impl GameState {
    pub fn new() -> GameState {
        let player_entity_id = 0;
        let map = Map::new(0.0, Coords::origin(), (TEST_SPAWN_POINTS.0.to_vec(), TEST_SPAWN_POINTS.1.to_vec()));
        let player = Player::new(player_entity_id, 0, Class::Warrior);
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
}

impl GameState {
    pub fn next_round(&mut self) {
        self.wave += 1;
        self.bounty = BASE_BOUNTY + (BOUNTY_GROWTH * self.wave as f64) as usize;
        self.populate();
        println!("Starting wave {}", self.wave);
    }

    pub fn spawn_entity(&mut self, mut entity: Entity) {
        entity.id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(entity);
    }

    pub fn round_finished(&self) -> bool {
        self.entities.iter()
            .filter(|e| e.is_monster() && e.team == Team::Monsters)
            .next().is_none()
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
