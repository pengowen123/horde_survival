use entity::{Entity, EntityType};

pub fn filter_entities(entities: &mut Vec<Entity>) {
    *entities = entities.iter().cloned().filter(|e| {
        let result;

        if e.is_enemy {
            result = !e.is_dead();
        } else if e.entity_type == EntityType::Player {
            result = true;
        } else {
            result = true;
        }

        result && !(e.lifetime == 1)
    }).collect();
}
