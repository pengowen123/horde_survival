use consts::*;
use hsgraphics::*;
use hsgraphics::object3d::Object3d;
use hsgraphics::utils::*;
use entity::Entity;

impl GraphicsState {
    pub fn update_entity_objects(&mut self, entities: &[Entity], player_entity_id: usize) {
        // TODO: Don't remove all objects, only remove them if their entity was updated
        self.remove_objects3d(ENTITY_OBJECT_ID);

        for entity in entities {
            //if entity.id == player_entity_id || !entity.needs_update{
                //continue;
            //}
            if entity.id == player_entity_id {
                continue;
            }

            let texture = self.get_texture(get_texture_id(&entity.entity_type));
            let size = get_entity_box_size(&entity.entity_type);
            let coords = get_unscaled_cube_coords(&entity.coords);
            let (v, i) = shapes3d::cube(coords, size);

            let mut cube_object = Object3d::from_slice(&mut self.factory, &v, &i, texture);

            cube_object.id = ENTITY_OBJECT_ID;
            self.objects3d.push(cube_object);
        }
    }
}
