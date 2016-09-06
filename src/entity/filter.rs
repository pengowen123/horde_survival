use entity::{Entity, EntityType};

pub fn filter_entities(entities: &mut Vec<Entity>) {
    *entities = entities.iter().cloned().filter(|e| {
        let result;

        if e.entity_type == EntityType::Player || e.is_dummy() {
            result = true;
        } else {
            result = !e.is_dead();
        }

        let x = result && !(e.lifetime == 1);

        if !x {
            debug!("entity removed: type: {:?}, health: {:?}, lifetime: {:?}", e.entity_type, e.health, e.lifetime);
        }

        x
    }).collect();
}
