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

pub fn get_closest_entity(index: usize, entities: &[Entity]) -> Option<(usize, f64)> {
    let entity = &entities[index];

    let mut closest_index = 0;
    let mut closest_distance = None;

    for (i, e) in entities.iter().enumerate() {
        if e.is_dummy() || !e.is_enemy_of(&entity) {
            continue;
        }

        let distance = e.coords.distance(&entity.coords);

        if let Some(ref mut d) = closest_distance {
            let x: f64 = *d;
            *d = x.max(distance);
        } else {
            closest_distance = Some(distance);
            closest_index = i;
        }
    }

    if let Some(d) = closest_distance {
        Some((closest_index, d))
    } else {
        None
    }
}
