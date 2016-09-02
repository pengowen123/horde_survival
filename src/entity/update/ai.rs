use player::Player;
use entity::*;
use consts::entity::*;

pub fn apply_ai(target_index: usize, entities: &mut Vec<Entity>, next_id: &mut usize, player: &mut Player) {
    let mut closest_index = None;
    let mut closest_distance = None;

    // Scoped for mutable borrow
    {
        let entity = &entities[target_index];
        let coords = &entity.coords;

        for (i, e) in entities.iter().enumerate().filter(|&(_, e)| e.is_enemy_of(entity)) {
            let distance = e.coords.distance(coords);

            match closest_distance {
                Some(d) => {
                    if distance < d {
                        closest_distance = Some(distance);
                        closest_index = Some(i);
                    }
                },
                None => {
                    closest_distance = Some(distance);
                    closest_index = Some(i);
                }
            }
        }
    }

    let mut attack = false;

    if let Some(i) = closest_index {
        let distance = closest_distance.unwrap();
        let coords = entities[i].coords.clone();
        let entity = &mut entities[target_index];
        let range = entity.current_weapon.get_real_range();

        entity.direction = entity.coords.direction_to(&coords);

        if distance <= range {
            attack = true;
        }

        if distance >= range * AI_RANGE_THRESHOLD {
            entity.move_forward(0.0);
        }
    }

    if attack {
        try_attack(target_index, entities, next_id, player);
    }
}
