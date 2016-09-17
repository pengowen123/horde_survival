use gfx::{self, tex};
use gfx::traits::FactoryExt;
use gfx_device_gl;

use consts::graphics::*;
use hsgraphics::*;
use hsgraphics::utils::get_camera;
use hsgraphics::object2d::Object2d;
use hsgraphics::object3d::{Object3d, ShaderView};
use entity::Entity;
use minimap::Minimap;
use world::Coords;

pub struct GraphicsState {
    // Window state variables
    pub window_position: (i32, i32),
    pub window_center: (i32, i32),

    // Objects
    pub objects2d: Vec<Object2d>,
    pub objects3d: Vec<Object3d>,

    // PSO's
    pub pso2d: object2d::ObjectPSO,
    pub pso3d: object3d::ObjectPSO,

    // Textures
    pub textures: Vec<ShaderView>,

    // Constants
    pub aspect_ratio: f32,

    // Misc
    pub minimap: Minimap,
    pub sampler: gfx::handle::Sampler<gfx_device_gl::Resources>,
    pub camera: [[f32; 4]; 4],
}

// Constructor
impl GraphicsState {
    pub fn new<F>(factory: &mut F) -> GraphicsState
        where F: gfx::Factory<gfx_device_gl::Resources>
    {
        let pso2d = match factory.create_pipeline_simple(
            include_bytes!("../include/triangle/shader/triangle_150.glslv"),
            include_bytes!("../include/triangle/shader/triangle_150.glslf"),
            gfx2d::pipe::new()) {
                Ok(p) => p,
                Err(e) => crash!("Failed to create 2d PSO: {}", e),
            };

        let pso3d = match factory.create_pipeline_simple(
            include_bytes!("../include/cube/shader/cube_150.glslv"),
            include_bytes!("../include/cube/shader/cube_150.glslf"),
            gfx3d::pipe::new()) {
                Ok(p) => p,
                Err(e) => crash!("Failed to create 3d PSO: {}", e),
            };

        let textures = vec![
            texture::create_texture(factory, &[FLOOR_TEXELS]),
            texture::create_texture(factory, &[ZOMBIE_TEXELS])
        ];

        let sampler_info = tex::SamplerInfo::new(tex::FilterMethod::Bilinear, tex::WrapMode::Clamp);
        let sampler = factory.create_sampler(sampler_info);
        let aspect_ratio = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

        GraphicsState {
            window_position: (0, 0),
            window_center: (0, 0),
            objects2d: Vec::new(),
            objects3d: Vec::new(),
            pso2d: pso2d,
            pso3d: pso3d,
            minimap: Minimap::new(MINIMAP_SCALE),
            aspect_ratio: aspect_ratio,
            sampler: sampler,
            camera: get_camera(Coords::origin(), START_CAMERA_ANGLE, aspect_ratio),
            textures: textures,
        }
    }
}

// Object methods (2d)
impl GraphicsState {
    pub fn remove_objects2d(&mut self, id: usize) {
        self.objects2d = self.objects2d.iter().cloned().filter(|o| o.id != id).collect();
    }

    pub fn encode_objects2d(&self, encoder: &mut ObjectEncoder) {
        for object in &self.objects2d {
            object.encode(encoder, &self.pso2d);
        }
    }
}

// Object methods (3d)
impl GraphicsState {
    pub fn add_object3d(&mut self, mut object: Object3d, id: usize) {
        object.id = id;
        self.objects3d.push(object);
    }

    pub fn encode_objects3d(&self, encoder: &mut ObjectEncoder) {
        for object in &self.objects3d {
            object.encode(encoder, &self.pso3d, self.camera);
        }
    }

    pub fn update_entity_objects(&mut self, entities: &[Entity]) {
        // TODO: Implement this (create objects for each entity)
    }
}

// Minimap
impl GraphicsState {
    pub fn update_minimap(&mut self, entities: &[Entity]) {
        self.minimap = Minimap::from_entities(entities, self.minimap.scale);
    }

    pub fn update_minimap_objects(&mut self, factory: &mut gfx_device_gl::Factory, color: &ObjectColor)
    {
        self.remove_objects2d(MINIMAP_OBJECT_ID);

        // TODO: Make minimap bounded, and draw borders
        for entity in &self.minimap.entities {
            let square = shapes2d::square(entity.coords,
                                        MINIMAP_ENTITY_SIZE,
                                        entity.color.clone(),
                                        entity.direction.1 as f32);

            let mut square_object = Object2d::from_slice(factory, &square, color.clone());
            square_object.id = MINIMAP_OBJECT_ID;
            self.objects2d.push(square_object);
        }
    }
}

// Misc
impl GraphicsState {
    pub fn update_camera(&mut self, coords: Coords, direction: (f64, f64)) {
        self.camera = get_camera(coords, direction, self.aspect_ratio);
    }

    pub fn get_texture(&self, id: usize) -> ShaderView {
        match self.textures.get(id) {
            Some(t) => t.clone(),
            None => crash!("Failed to find texture with ID {}", id),
        }
    }
}
