use hsgraphics::GraphicsState;
use hsgraphics::texture::Texture;
use entity::{Entity, EntityType};

#[derive(Clone)]
pub struct MinimapEntity {
    pub coords: [f32; 2],
    pub direction: (f64, f64),
    pub texture: Texture,
    pub id: usize,
}

pub struct Minimap {
    pub entities: Vec<MinimapEntity>,
    pub next_id: usize,
    pub scale: f32,
}

impl MinimapEntity {
    pub fn new(coords: [f32; 2], direction: (f64, f64), texture: Texture) -> MinimapEntity {
        MinimapEntity {
            coords: coords,
            direction: direction,
            texture: texture,
            id: 0,
        }
    }

    pub fn from_entity(entity: &Entity, scale: f32, graphics: &GraphicsState) -> MinimapEntity {
        let coords = entity.coords.scaled(scale as f64);
        let texture = graphics.get_texture(get_minimap_entity_texture_id(&entity.entity_type));

        MinimapEntity::new([coords.x as f32, coords.z as f32],
                           entity.direction.clone(),
                           texture)
    }
}

// Constructors
impl Minimap {
    pub fn new(scale: f32) -> Minimap {
        Minimap {
            entities: Vec::new(),
            next_id: 0,
            scale: scale,
        }
    }

    pub fn from_entities(entities: &[Entity], scale: f32, graphics: &GraphicsState) -> Minimap {
        let mut minimap = Minimap::new(scale);

        for entity in entities {
            minimap.add_entity(MinimapEntity::from_entity(entity, scale, graphics));
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
fn get_minimap_entity_texture_id(entity_type: &EntityType) -> usize {
    match *entity_type {
        EntityType::Player => 5,
        EntityType::Zombie => 6,
        EntityType::FlyingBallLinear => 7,
        EntityType::FlyingBallArc => 8,
    }
}
