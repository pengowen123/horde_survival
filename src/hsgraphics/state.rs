use gfx::Device;
use glutin::Window;
use cgmath::Matrix4;
use gfx_device_gl;

use hsgraphics::object2d::Object2d;
use hsgraphics::object3d::*;
use hsgraphics::*;
use gamestate::GameState;
use assets::AssetLoader;
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
    pub pso_gui: object_gui::ObjectPSO,
    pub data3d: gfx3d::pipe::Data<gfx_device_gl::Resources>,
    pub data2d: gfx2d::pipe::Data<gfx_device_gl::Resources>,
    pub data_gui: gfx_gui::pipe::Data<gfx_device_gl::Resources>,

    // Glyph cache
    pub cache: TextCache,

    // Assets
    pub assets: AssetLoader<String>,

    // Controls
    pub last_cursor_pos: (i32, i32),
    pub camera: Matrix4<f32>,

    // Misc
    pub pixel_size: (f32, f32),
    pub dpi: f32,
}

// Updates
impl GraphicsState {
    /// Draws the stored objects to the window
    pub fn draw_game(&mut self, window: &Window) {
        // Clear the encoder
        self.encoder.clear(&self.data3d.out_color, CLEAR_COLOR);
        self.encoder.clear_depth(&self.data3d.out_depth, 1.0);

        // Draw the objects
        // NOTE: 3d objects must be drawn first (to let the GUI not be drawn over)
        self.encode_objects3d();
        self.encode_objects2d();

        // Send commands to the device
        self.encoder.flush(&mut self.device);

        // Display the results in the window
        if let Err(e) = window.swap_buffers() {
            error!("Failed to swap buffers: {}", e);
        }

        // Cleanup
        self.device.cleanup();
    }

    /// Update the graphics state to reflect changes in the game state
    pub fn update(&mut self, game: &GameState) {
        if self.options.crosshair {
            self.update_crosshair();
        }
        self.update_entity_objects(&game.entities, game.player.entity_id);
    }

    /// Like GraphicsState::draw_game, but it assumes that clearing and drawing has already been
    /// done
    pub fn draw_gui(&mut self, window: &Window) {
        self.encoder.flush(&mut self.device);

        if let Err(e) = window.swap_buffers() {
            error!("Failed to swap buffers: {}", e);
        }

        self.device.cleanup();
    }

    /// Updates the camera to the provided one
    pub fn update_camera(&mut self, camera: Camera) {
        self.camera = camera.into_matrix(self.aspect_ratio);
        let locals = gfx3d::Locals { transform: self.camera.into() };
        self.encoder.update_constant_buffer(&self.data3d.locals, &locals);
    }

    /// Updates the DPI
    pub fn update_dpi(&mut self, window: &Window) {
        self.dpi = window.hidpi_factor();
    }

    /// Updates the crosshair
    pub fn update_crosshair(&mut self) {
        self.remove_objects2d(CROSSHAIR_OBJECT_ID);

        if self.options.crosshair {
            let mut vertices = shape!([1.0, 0.0],
                                      [1.0, 0.0],
                                      [0.0, 1.0],
                                      [0.0, 1.0],
                                      [-1.0, 0.0],
                                      [1.0, 1.0],
                                      [1.0, 0.0],
                                      [1.0, 0.0],
                                      [-1.0, 0.0],
                                      [1.0, 1.0],
                                      [0.0, -1.0],
                                      [0.0, 0.0]);

            let (scale_x, scale_y) = (CROSSHAIR_SIZE / self.window_size.0 as f32,
                                      CROSSHAIR_SIZE / self.window_size.1 as f32);

            for v in &mut vertices {
                v.pos[0] *= scale_x;
                v.pos[1] *= scale_y;
            }

            let texture = self.assets
                .get_or_load_texture("crosshair", &mut self.factory)
                .unwrap_or_else(|e| crash!("Failed to get texture: crosshair ({})", e))
                .clone();
            let object = Object2d::new(&mut self.factory, &vertices, texture, ());
            self.add_object2d(object, CROSSHAIR_OBJECT_ID);
        }
    }
}

impl GraphicsState {
    /// Adds the given 2d object, and sets its ID
    pub fn add_object2d(&mut self, mut object: Object2d, id: usize) {
        object.id = id;
        self.objects2d.push(object);
    }

    /// Removes all 2d objects with the given ID
    pub fn remove_objects2d(&mut self, id: usize) {
        self.objects2d = self.objects2d.iter().cloned().filter(|o| o.id != id).collect();
    }

    /// Draws all stored 2d objects
    pub fn encode_objects2d(&mut self) {
        for object in &self.objects2d {
            object.encode(&mut self.encoder, &self.pso2d, &mut self.data2d);
        }
    }
}

impl GraphicsState {
    /// Adds the given 3d object, and sets its ID
    pub fn add_object3d(&mut self, mut object: Object3d, id: usize) {
        object.id = id;
        self.objects3d.push(object);
    }

    /// Removes all 3d objects with the given ID
    pub fn remove_objects3d(&mut self, id: usize) {
        self.objects3d = self.objects3d.iter().cloned().filter(|o| o.id != id).collect();
    }

    /// Draws all stored 3d objects
    pub fn encode_objects3d(&mut self) {
        for object in &self.objects3d {
            object.encode(&mut self.encoder, &self.pso3d, &mut self.data3d);
        }
    }
}

// Misc
impl GraphicsState {
    /// Moves the cursor to the window center
    pub fn reset_cursor(&mut self, window: &Window) {
        self.last_cursor_pos = self.window_center;
        let (x, y) = (self.window_center.0, self.window_center.1);

        if window.set_cursor_position(x, y).is_err() {
            error!("Failed to set cursor position to ({}, {})", x, y);
        }
    }
}
