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
}

impl GameState {
    pub fn new(player: Player, map: Map, player_coords: Coords, player_team: Team) -> GameState {
        GameState {
            entities: vec![Entity::player(player_coords, player.entity_id, player_team)],
            next_entity_id: player.entity_id + 1,
            player: player,
            map: map,
        }
    }
}

impl GameState {
    pub fn spawn_entity(&mut self, mut entity: Entity) {
        entity.id = self.next_entity_id;
        self.next_entity_id += 1;
        self.entities.push(entity);
    }
}
