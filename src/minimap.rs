use hsgraphics::GraphicsState;
use hsgraphics::texture::Texture;
use entity::{Entity, EntityType};
use hslog::CanUnwrap;

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

    pub fn from_entity(entity: &Entity, scale: f32, graphics: &mut GraphicsState) -> MinimapEntity {
        let coords = entity.coords.scaled(scale as f64);
        let name = get_minimap_entity_texture_name(&entity.entity_type);
        let texture = unwrap_or_log!(graphics.assets.get_or_load_texture(name, &mut graphics.factory),
                                     "Failed to load texture: {}", name);

        MinimapEntity::new([coords.x as f32, coords.z as f32],
                           entity.direction,
                           texture.clone())
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

    pub fn from_entities(entities: &[Entity], scale: f32, graphics: &mut GraphicsState) -> Minimap {
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
fn get_minimap_entity_texture_name(entity_type: &EntityType) -> &str {
    match *entity_type {
        EntityType::Player => "minimap_player",
        EntityType::Zombie => "minimap_zombie",
        EntityType::FlyingBallLinear => "minimap_ball_linear",
        EntityType::FlyingBallArc => "minimap_ball_arc",
    }
}
