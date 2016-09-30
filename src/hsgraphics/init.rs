use glutin::{self, Window, GlRequest};
use gfx::{self, tex, Factory};
use gfx::traits::FactoryExt;
use gfx_window_glutin;

use consts::*;
use hsgraphics::*;
use hsgraphics::object3d::*;
use gamestate::GameState;
use minimap::Minimap;

impl GraphicsState {
    pub fn new(options: GraphicsOptions, game: &GameState) -> (GraphicsState, Window) {
        let gl = GlRequest::GlThenGles {
            opengles_version: (2, 0),
            opengl_version: (2, 1),
        };

        let mut builder = glutin::WindowBuilder::new()
            .with_title(WINDOW_NAME)
            .with_dimensions(WINDOW_WIDTH, WINDOW_HEIGHT)
            .with_gl(gl);

        if options.fullscreen {
            builder = builder.with_fullscreen(glutin::get_primary_monitor());
        }

        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<gfx3d::ColorFormat, gfx3d::DepthFormat>(builder);
        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        let pso2d = match factory.create_pipeline_simple(
            include_bytes!("../include/vertex.glsl"),
            include_bytes!("../include/fragment.glsl"),
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

        let textures = create_all_textures(&mut factory);

        let sampler_info = tex::SamplerInfo::new(tex::FilterMethod::Bilinear, tex::WrapMode::Clamp);
        let sampler = factory.create_sampler(sampler_info);
        let aspect_ratio = WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32;
        let camera = get_camera(game.map.player_spawn.clone(), START_CAMERA_ANGLE, aspect_ratio);

        let vbuf = factory.create_vertex_buffer(&[]);
        let texture = load_texture_raw(&mut factory, [2, 2], &[0; 4]);
        
        let data = gfx3d::pipe::Data {
            vbuf: vbuf,
            transform: [[0.0; 4]; 4],
            locals: factory.create_constant_buffer(1),
            color: (texture.clone(), sampler.clone()),
            out_color: main_color.clone(),
            out_depth: main_depth,
        };

        let vbuf2d = factory.create_vertex_buffer(&[]);

        let data2d = gfx2d::pipe::Data {
            vbuf: vbuf2d,
            color: (texture, sampler),
            out: main_color,
        };

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
            data: data,
            data2d: data2d,
            minimap: Minimap::new(MINIMAP_SCALE),
            aspect_ratio: aspect_ratio,
            camera: camera,
            textures: textures,
            device: device,
            last_cursor_pos: WINDOW_CENTER,
            pixel_size: (1.0 / WINDOW_WIDTH as f32, 1.0 / WINDOW_HEIGHT as f32),
        };

        let texture = state.get_texture(0);
        let (v, i) = shapes3d::plane(FLOOR_HEIGHT, 1000.0);
        let floor_object = Object3d::from_slice(&mut state.factory, &v, &i, texture);

        state.add_object3d(floor_object, 0);

        (state, window)
    }
}
