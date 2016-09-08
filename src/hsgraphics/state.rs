use gfx::traits::FactoryExt;
use gfx_device_gl::Factory;

use consts::graphics::minimap::*;
use hsgraphics::*;
use entity::Entity;
use minimap::Minimap;

pub struct GraphicsState {
    // Window state variables
    pub window_position: (i32, i32),
    pub window_center: (i32, i32),

    // Objects
    pub objects: Vec<Object>,
    pub next_object_id: usize,

    // PSO's
    pub pso: Vec<ObjectPSO>,

    // Minimap
    pub minimap: Minimap,
}

// Constructor
impl GraphicsState {
    pub fn new(factory: &mut Factory) -> GraphicsState {
        let mut pso = Vec::new();

        pso.push(
            match factory.create_pipeline_simple(
                include_bytes!("../include/shader/triangle_150.glslv"),
                include_bytes!("../include/shader/triangle_150.glslf"),
                pipe::new()) {
                    Ok(x) => x,
                    Err(e) => crash!("{}", e),
            }
        );

        GraphicsState {
            window_position: (0, 0),
            window_center: (0, 0),
            objects: Vec::new(),
            next_object_id: 1,
            pso: pso,
            minimap: Minimap::new(MINIMAP_SCALE),
        }
    }
}

// Misc
impl GraphicsState {
    pub fn get_pso(&self, id: usize) -> &ObjectPSO {
        match self.pso.get(id) {
            Some(o) => o,
            None => crash!("Failed to find PSO with id: {}", id),
        }
    }
}

// Object methods
impl GraphicsState {
    pub fn add_object(&mut self, mut object: Object) {
        object.id = self.next_object_id;
        self.next_object_id += 1;
        self.objects.push(object);
    }

    pub fn remove_objects(&mut self, id: usize) {
        self.objects = self.objects.iter().cloned().filter(|o| o.id != id).collect();
    }

    pub fn encode_objects(&self, encoder: &mut ObjectEncoder) {
        for object in &self.objects {
            object.encode(encoder, self.get_pso(object.pso_id));
        }
    }
}

// Minimap
impl GraphicsState {
    pub fn update_minimap(&mut self, entities: &[Entity]) {
        self.minimap = Minimap::from_entities(entities, self.minimap.scale);
    }

    pub fn update_minimap_objects(&mut self, factory: &mut Factory, color: &ObjectColor) {
        self.remove_objects(MINIMAP_OBJECT_ID);

        // TODO: Make minimap bounded, and draw borders
        for entity in &self.minimap.entities {
            let square = shapes::square(entity.coords, MINIMAP_ENTITY_SIZE, entity.color.clone());
            let square_object = Object::from_slice(0, factory, &square, color.clone());

            self.objects.push(square_object);
        }
    }
}
