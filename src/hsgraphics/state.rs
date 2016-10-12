use gfx::Device;
use glutin::Window;
use cgmath::Matrix4;
use gfx_device_gl;

use hsgraphics::object2d::Object2d;
use hsgraphics::object3d::*;
use hsgraphics::*;
use gamestate::GameState;
use assets::AssetLoader;
use minimap::Minimap;
use world::Coords;
use hslog::CanUnwrap;
use consts::*;

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

    // gfx things
    pub pso2d: object2d::ObjectPSO,
    pub pso3d: object3d::ObjectPSO,
    pub data: gfx3d::pipe::Data<gfx_device_gl::Resources>,
    pub data2d: gfx2d::pipe::Data<gfx_device_gl::Resources>,

    // Glyph cache
    pub cache: GlyphCache,

    // Assets
    pub assets: AssetLoader<String>,

    // Minimap
    pub minimap: Minimap,

    // Controls
    pub last_cursor_pos: (i32, i32),
    pub camera: Matrix4<f32>,

    // Misc
    pub pixel_size: (f32, f32),
    pub dpi: f32,
}

// Updates
impl GraphicsState {
    pub fn draw(&mut self, window: &Window) {
        self.encoder.clear(&self.data.out_color, CLEAR_COLOR);
        self.encoder.clear_depth(&self.data.out_depth, 1.0);
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

    pub fn update_camera(&mut self, coords: Coords, direction: (f64, f64)) {
        self.camera = get_camera(coords, direction, self.aspect_ratio);
        let locals = gfx3d::Locals { transform: self.camera.into() };
        self.encoder.update_constant_buffer(&self.data.locals, &locals);
    }

    pub fn update_dpi(&mut self, window: &Window) {
        self.dpi = window.hidpi_factor();
    }

    pub fn update_crosshair(&mut self) {
        self.remove_objects2d(CROSSHAIR_OBJECT_ID);

        if self.options.crosshair {
            let mut vertices = shape!(
                [1.0, 0.0], [1.0, 0.0],
                [0.0, 1.0], [0.0, 1.0],
                [-1.0, 0.0], [1.0, 1.0],
                [1.0, 0.0], [1.0, 0.0],
                [-1.0, 0.0], [1.0, 1.0],
                [0.0, -1.0], [0.0, 0.0]
            );

            let (scale_x, scale_y) = (CROSSHAIR_SIZE / self.window_size.0 as f32,
                                      CROSSHAIR_SIZE / self.window_size.1 as f32);

            for v in &mut vertices {
                v.pos[0] *= scale_x;
                v.pos[1] *= scale_y;
            }
   
            let texture = unwrap_or_log!(self.assets.get_or_load_texture("crosshair", &mut self.factory),
                                         "Failed to get texture: crosshair").clone();
            let object = Object2d::from_slice(&mut self.factory, &vertices, texture);
            self.add_object2d(object, CROSSHAIR_OBJECT_ID);
        }
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
            object.encode(&mut self.encoder, &self.pso2d, &mut self.data2d);
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
            object.encode(&mut self.encoder, &self.pso3d, &mut self.data);
        }
    }
}

// Misc
impl GraphicsState {
    pub fn get_scales(&self, d: f32) -> (f32, f32) {
        (d * MINIMAP_SCALE / self.window_size.0 as f32,
         d * MINIMAP_SCALE / self.window_size.1 as f32)
    }

    pub fn reset_cursor(&mut self, window: &Window) {
        // NOTE: This doesn't do anything useful right now, maybe I will fix it in the future
        self.last_cursor_pos = self.window_center;
        let (x, y) = (self.window_center.0, self.window_center.1);

        if let Err(_) = window.set_cursor_position(x, y) {
            error!("Failed to set cursor position to ({}, {})", x, y);
        }
    }
}
