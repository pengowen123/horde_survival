use consts::balance::*;
use entity::flags::Team;
use entity::Entity;
use player::Player;
use map::Map;
use world::Coords;

pub struct GameState {
    pub entities: Vec<Entity>,
    pub next_entity_id: usize,
    pub player: Player,
    pub map: Map,
    pub wave: usize,
    pub bounty: usize,
}

impl GameState {
    pub fn new(player: Player, map: Map, player_coords: Coords, player_team: Team) -> GameState {
        GameState {
            entities: vec![Entity::player(player_coords, player.entity_id, player_team)],
            next_entity_id: player.entity_id + 1,
            player: player,
            map: map,
            wave: 1,
            bounty: BASE_BOUNTY,
        }
    }
}

impl GameState {
    pub fn next_round(&mut self) {
        self.wave += 1;
        self.bounty = BASE_BOUNTY + (BOUNTY_GROWTH * self.wave as f64) as usize;
    }

    pub fn spawn_entity(&mut self, mut entity: Entity) {
        entity.id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(entity);
    }
}
