use gfx::{self, tex, Factory};
use gfx::traits::FactoryExt;
use cgmath::Matrix4;
use glutin::{self, Window};
use {gfx_device_gl, gfx_window_glutin};
use gfx::Device;

use consts::*;
use hsgraphics::*;
use hsgraphics::utils::*;
use hsgraphics::object2d::Object2d;
use hsgraphics::object3d::*;
use hsgraphics::gfx2d::Vertex;
use entity::Entity;
use minimap::Minimap;
use world::Coords;
use gamestate::GameState;

pub struct GraphicsState {
    // Options
    pub options: GraphicsOptions,

    // Window state variables
    pub window_size: (u32, u32),
    pub window_center: (i32, i32),
    pub aspect_ratio: f32,
    pub factory: gfx_device_gl::Factory,
    pub encoder: ObjectEncoder,
    pub device: gfx_device_gl::Device,
    pub should_close: bool,

    // Objects
    pub objects2d: Vec<Object2d>,
    pub objects3d: Vec<Object3d>,

    // PSO's
    pub pso2d: object2d::ObjectPSO,
    pub pso3d: object3d::ObjectPSO,

    // Textures
    pub textures: Vec<ShaderView>,

    // Minimap
    pub minimap: Minimap,

    // Controls
    pub last_cursor_pos: (i32, i32),
    pub camera: Matrix4<f32>,

    // Misc
    pub sampler: gfx::handle::Sampler<gfx_device_gl::Resources>,
    pub main_color: Object3dColor,
    pub main_depth: Object3dDepth,
    pub pixel_size: (f32, f32),
}

// Constructor
impl GraphicsState {
    pub fn new(options: GraphicsOptions, game: &GameState) -> (GraphicsState, Window) {
        let builder = glutin::WindowBuilder::new()
            .with_title(WINDOW_NAME)
            .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT);

        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<gfx3d::ColorFormat, gfx3d::DepthFormat>(builder);
        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

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
            // NOTE: If the order of these changes, also update get_entity_texture
            texture::create_texture(&mut factory, &[TEXELS_FLOOR]),
            texture::create_texture(&mut factory, &[TEXELS_PLAYER]),
            texture::create_texture(&mut factory, &[TEXELS_ZOMBIE]),
            texture::create_texture(&mut factory, &[TEXELS_FLYING_BALL_LINEAR]),
            texture::create_texture(&mut factory, &[TEXELS_FLYING_BALL_ARC]),
        ];

        let sampler_info = tex::SamplerInfo::new(tex::FilterMethod::Bilinear, tex::WrapMode::Clamp);
        let sampler = factory.create_sampler(sampler_info);
        let aspect_ratio = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;

        let mut state = GraphicsState {
            factory: factory,
            encoder: encoder,
            options: options,
            objects2d: Vec::new(),
            objects3d: Vec::new(),
            window_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            window_center: WINDOW_CENTER,
            should_close: false,
            pso2d: pso2d,
            pso3d: pso3d,
            minimap: Minimap::new(MINIMAP_SCALE),
            aspect_ratio: aspect_ratio,
            sampler: sampler,
            camera: get_camera(game.map.player_spawn.clone(), START_CAMERA_ANGLE, aspect_ratio),
            textures: textures,
            main_color: main_color,
            main_depth: main_depth,
            device: device,
            last_cursor_pos: WINDOW_CENTER,
            pixel_size: (1.0 / WINDOW_WIDTH as f32, 1.0 / WINDOW_HEIGHT as f32),
        };

        let texture = state.get_texture(0);
        let (i, v) = shapes3d::plane(FLOOR_HEIGHT, 1000.0);
        let floor_object = Object3d::from_slice(&mut state.factory,
                                                (&i, &v),
                                                state.main_color.clone(),
                                                state.main_depth.clone(),
                                                texture,
                                                state.sampler.clone());

        state.add_object3d(floor_object, 0);

        (state, window)
    }
}

// Updates
impl GraphicsState {
    pub fn draw(&mut self, window: &Window) {
        self.encoder.clear(&self.main_color, CLEAR_COLOR);
        self.encoder.clear_depth(&self.main_depth, 1.0);
        self.encode_objects3d();
        self.encode_objects2d();
        self.encoder.flush(&mut self.device);

        if let Err(e) = window.swap_buffers() {
            error!("Failed to swap buffers: {}", e);
        }

        self.device.cleanup();
    }

    pub fn update(&mut self, game: &GameState) {
        self.update_crosshair();
        self.update_minimap(&game.entities);
        self.update_minimap_objects();
        self.update_entity_objects(&game.entities, game.player.entity_id);
    }

    pub fn draw_gui(&mut self, window: &Window) {
        self.encoder.flush(&mut self.device);

        if let Err(e) = window.swap_buffers() {
            error!("Failed to swap buffers: {}", e);
        }

        self.device.cleanup();
    }
}

// Object methods (2d)
impl GraphicsState {
    pub fn add_object2d(&mut self, mut object: Object2d, id: usize) {
        object.id = id;
        self.objects2d.push(object);
    }

    pub fn remove_objects2d(&mut self, id: usize) {
        self.objects2d = self.objects2d.iter().cloned().filter(|o| o.id != id).collect();
    }

    pub fn encode_objects2d(&mut self) {
        for object in &self.objects2d {
            object.encode(&mut self.encoder, &self.pso2d);
        }
    }
}

// Object methods (3d)
impl GraphicsState {
    pub fn add_object3d(&mut self, mut object: Object3d, id: usize) {
        object.id = id;
        self.objects3d.push(object);
    }

    pub fn remove_objects3d(&mut self, id: usize) {
        self.objects3d = self.objects3d.iter().cloned().filter(|o| o.id != id).collect();
    }

    pub fn encode_objects3d(&mut self) {
        for object in &self.objects3d {
            object.encode(&mut self.encoder, &self.pso3d, self.camera.into());
        }
    }

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

            let mut cube_object = Object3d::from_slice(&mut self.factory,
                                                       (&v, &i),
                                                       self.main_color.clone(),
                                                       self.main_depth.clone(),
                                                       texture,
                                                       self.sampler.clone());

            cube_object.id = ENTITY_OBJECT_ID;
            self.objects3d.push(cube_object);
        }
    }
}

// Minimap
impl GraphicsState {
    pub fn update_minimap(&mut self, entities: &[Entity]) {
        if self.options.minimap_enabled {
            self.minimap = Minimap::from_entities(entities, self.minimap.scale);
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
                                              entity.color.clone(),
                                              entity.direction.1 as f32,
                                              self.get_scales(MINIMAP_ENTITY_SIZE));

            for vertex in &mut square {
                vertex.pos[0] += MINIMAP_LOCATION.0;
                vertex.pos[1] += MINIMAP_LOCATION.1;
            }

            let mut square_object = Object2d::from_slice(&mut self.factory, &square, self.main_color.clone());
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

    pub fn update_crosshair(&mut self) {
        self.remove_objects2d(CROSSHAIR_OBJECT_ID);

        if self.options.crosshair {
            let mut vertices = [
                Vertex { pos: [1.0, 0.0], color: CROSSHAIR_COLOR },
                Vertex { pos: [0.0, 1.0], color: CROSSHAIR_COLOR },
                Vertex { pos: [-1.0, 0.0], color: CROSSHAIR_COLOR },
                Vertex { pos: [1.0, 0.0], color: CROSSHAIR_COLOR },
                Vertex { pos: [-1.0, 0.0], color: CROSSHAIR_COLOR },
                Vertex { pos: [0.0, -1.0], color: CROSSHAIR_COLOR },
            ];

            let (scale_x, scale_y) = (CROSSHAIR_SIZE / self.window_size.0 as f32,
                                      CROSSHAIR_SIZE / self.window_size.1 as f32);
            for v in vertices.iter_mut() {
                v.pos[1] += CROSSHAIR_VERTICAL_OFFSET;
                v.pos[0] *= scale_x;
                v.pos[1] *= scale_y;
            }

            let object = Object2d::from_slice(&mut self.factory, &vertices, self.main_color.clone());
            self.add_object2d(object, CROSSHAIR_OBJECT_ID);
        }
    }

    pub fn get_scales(&self, d: f32) -> (f32, f32) {
        (d * MINIMAP_SCALE / self.window_size.0 as f32,
         d * MINIMAP_SCALE / self.window_size.1 as f32)
    }
}
