use entity::{Entity, EntityType};

pub fn filter_entities(entities: &mut Vec<Entity>) {
    *entities = entities.iter().cloned().filter(|e| {
        let result;

        if e.entity_type == EntityType::Player || e.is_dummy() {
            result = true;
        } else {
            result = !e.is_dead();
        }

        let keep = result && !(e.lifetime == 1);

        if !keep {
            debug!("Entity removed by filter: ID {}: {:?}", e.id, e.entity_type);
        }

        keep
    }).collect();
}
