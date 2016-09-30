use consts::graphics::minimap::*;
use hsgraphics::object2d::Object2d;
use hsgraphics::*;
use minimap::Minimap;
use entity::Entity;

impl GraphicsState {
    pub fn update_minimap(&mut self, entities: &[Entity]) {
        if self.options.minimap_enabled {
            self.minimap = Minimap::from_entities(entities, self.minimap.scale, self);
        }
    }

    pub fn update_minimap_objects(&mut self)
    {
        if !self.options.minimap_enabled {
            return;
        }

        self.remove_objects2d(MINIMAP_OBJECT_ID);

        // TODO: Make minimap bounded, and draw borders
        for entity in &self.minimap.entities {
            let mut square = shapes2d::square(entity.coords,
                                              MINIMAP_ENTITY_SIZE,
                                              entity.direction.1 as f32,
                                              self.get_scales(MINIMAP_ENTITY_SIZE));

            for vertex in &mut square {
                vertex.pos[0] += MINIMAP_LOCATION.0;
                vertex.pos[1] += MINIMAP_LOCATION.1;
            }

            let mut square_object = Object2d::from_slice(&mut self.factory,
                                                         &square,
                                                         entity.texture.clone());

            square_object.id = MINIMAP_OBJECT_ID;
            self.objects2d.push(square_object);
        }
    }
}
