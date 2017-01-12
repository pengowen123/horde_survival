use consts::*;
use hsgraphics::*;
use hsgraphics::object3d::Object3d;
use hsgraphics::utils::*;
use entity::Entity;

impl GraphicsState {
    /// Updates the internal entity list
    pub fn update_entity_objects(&mut self, entities: &[Entity], player_entity_id: usize) {
        // Removes outdated entity objects
        // TODO: Don't remove all objects, only remove them if their entity was updated
        self.remove_objects3d(ENTITY_OBJECT_ID);

        for entity in entities {
            //if entity.id == player_entity_id || !entity.needs_update{
                //continue;
            //}
            // Don't render the player entity, they can't see themselves
            // TODO: Eventually they might be able to (such as in multiplayer), so this check should
            //       be removed eventually
            if entity.id == player_entity_id {
                continue;
            }

            // Get the texture for the entity
            let name = get_texture_name(&entity.entity_type);
            let texture = self.assets.get_or_load_texture(name, &mut self.factory).unwrap_or_else(|e|
                                        crash!("Failed to load texture {} for entity type: {:?}
                                               ({})", name, entity.entity_type, e));

            // Get the entity size
            let size = get_entity_box_size(&entity.entity_type);
            // Get the coordinates of the entity model
            let coords = get_unscaled_cube_coords(&entity.coords);
            // Get the vertices of the object
            let (v, i) = shapes3d::cube(coords, size);

            // Create the object and add it to the object list
            let mut cube_object = Object3d::from_slice(&mut self.factory, &v, &i, texture.clone());
            cube_object.id = ENTITY_OBJECT_ID;
            self.objects3d.push(cube_object);
        }
    }
}
