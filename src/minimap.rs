use hsgraphics::Color;
use consts::graphics::minimap::*;
use entity::{Entity, EntityType};

#[derive(Clone)]
pub struct MinimapEntity {
    pub coords: [f32; 2],
    pub color: Color,
    pub id: usize,
}

pub struct Minimap {
    pub entities: Vec<MinimapEntity>,
    pub next_id: usize,
    pub scale: f64,
}

impl MinimapEntity {
    pub fn new(coords: [f32; 2], color: Color) -> MinimapEntity {
        MinimapEntity {
            coords: coords,
            color: color,
            id: 0,
        }
    }

    pub fn from_entity(entity: &Entity, scale: f64) -> MinimapEntity {
        let coords = entity.coords.scaled(scale);

        MinimapEntity::new([coords.x as f32, coords.z as f32], get_minimap_entity_color(&entity.entity_type))
    }
}

// Constructors
impl Minimap {
    pub fn new(scale: f64) -> Minimap {
        Minimap {
            entities: Vec::new(),
            next_id: 0,
            scale: scale,
        }
    }

    pub fn from_entities(entities: &[Entity], scale: f64) -> Minimap {
        let mut minimap = Minimap::new(scale);

        for entity in entities {
            minimap.add_entity(MinimapEntity::from_entity(entity, scale));
        }

        minimap
    }
}

// Entities
impl Minimap {
    pub fn add_entity(&mut self, mut entity: MinimapEntity) {
        entity.id = self.next_id;
        self.next_id += 1;
        self.entities.push(entity);
    }
}

// Utility function
fn get_minimap_entity_color(entity_type: &EntityType) -> Color {
    match *entity_type {
        EntityType::Player => MINIMAP_COLOR_PLAYER,
        EntityType::Zombie => MINIMAP_COLOR_ZOMBIE,
        EntityType::FlyingBallLinear => MINIMAP_COLOR_FLYING_BALL_LINEAR,
        EntityType::FlyingBallArc => MINIMAP_COLOR_FLYING_BALL_ARC,
    }
}
