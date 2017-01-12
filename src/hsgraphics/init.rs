use glutin::{self, Window};
use gfx::{self, Factory, Primitive, tex};
use gfx::traits::FactoryExt;
use conrod::text::GlyphCache;
use gfx_window_glutin;

use consts::*;
use hsgraphics::*;
use hsgraphics::shaders::*;
use gamestate::GameState;

impl GraphicsState {
    /// Initializes the graphics state and returns the state and the game window
    pub fn new(options: GraphicsOptions, game: &GameState) -> (GraphicsState, Window) {
        // Get window measurements
        let width = options.window_size.0;
        let height = options.window_size.1;
        let center = (width as i32 / 2, height as i32 / 2);

        // Set builder options
        let mut builder = glutin::WindowBuilder::new().with_title(WINDOW_NAME);

        if options.fullscreen {
            let monitor = glutin::get_primary_monitor();
            let (width, height) = monitor.get_dimensions();
            builder = builder.with_fullscreen(monitor)
                .with_dimensions(width, height);
        } else {
            builder = builder.with_dimensions(width, height);
        }

        // Initialize gfx objects
        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<ColorFormat, gfx3d::DepthFormat>(builder);
        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        // Scoped because device is moved, and get_info borrows it
        {
            let info = device.get_info();

            // Log info about OpenGL
            info!("Vendor: {}", info.platform_name.vendor);
            info!("Renderer: {}", info.platform_name.renderer);
            info!("GL version: {:?}", info.version);
            info!("GLSL version: {:?}", info.shading_language);
        }

        // Load shaders
        let shader_assets_path = "test_assets/shaders";

        // 2d
        let vs_2d_path = get_shader_version_path(&device, shader_assets_path, VS_2D_PATH);
        let fs_2d_path = get_shader_version_path(&device, shader_assets_path, FS_2D_PATH);

        // 3d
        let vs_3d_path = get_shader_version_path(&device, shader_assets_path, VS_3D_PATH);
        let fs_3d_path = get_shader_version_path(&device, shader_assets_path, FS_3D_PATH);

        // GUI
        let vs_gui_path = get_shader_version_path(&device, shader_assets_path, VS_GUI_PATH);
        let fs_gui_path = get_shader_version_path(&device, shader_assets_path, FS_GUI_PATH);

        // Create PSOs with the shaders (and log about their creation)
        log_create_pso!("2d", vs_2d_path, fs_2d_path);
        let pso2d = unwrap_pretty!(load_pso(&mut factory,
                                            vs_2d_path,
                                            fs_2d_path,
                                            Primitive::TriangleList,
                                            gfx2d::pipe::new()));

        log_create_pso!("3d", vs_3d_path, fs_3d_path);
        let pso3d = unwrap_pretty!(load_pso(&mut factory,
                                            vs_3d_path,
                                            fs_3d_path,
                                            Primitive::TriangleList,
                                            gfx3d::pipe::new()));

        log_create_pso!("gui", vs_gui_path, fs_gui_path);
        let pso_gui = unwrap_pretty!(load_pso(&mut factory,
                                              vs_gui_path,
                                              fs_gui_path,
                                              Primitive::TriangleList,
                                              gfx_gui::pipe::new()));

        // Create texture sampler
        let sampler_info = tex::SamplerInfo::new(tex::FilterMethod::Bilinear, tex::WrapMode::Clamp);
        let sampler = factory.create_sampler(sampler_info);

        // Create initial camera
        let aspect_ratio = width as f32 / height as f32;
        let camera = initial_camera(game.map.player_spawn.clone(), aspect_ratio);

        // Create dummy textures and buffers
        let vbuf = factory.create_vertex_buffer(&[]);
        let vbuf2d = factory.create_vertex_buffer(&[]);
        let vbuf_gui = factory.create_vertex_buffer(&[]);
        let texture = load_texture_raw(&mut factory, [2, 2], &[0; 4]);

        // Initialize pipeline data
        let data3d = gfx3d::pipe::Data {
            vbuf: vbuf,
            transform: [[0.0; 4]; 4],
            locals: factory.create_constant_buffer(1),
            color: (texture.clone(), sampler.clone()),
            out_color: main_color.clone(),
            out_depth: main_depth,
        };

        let data2d = gfx2d::pipe::Data {
            vbuf: vbuf2d,
            color: (texture, sampler),
            out: main_color.clone(),
        };

        let data_gui = gfx_gui::pipe::Data {
            vbuf: vbuf_gui,
            out: main_color,
        };

        // Create glyph cache
        let dpi_factor = window.hidpi_factor();
        let dpi = dpi_factor as u32;
        let (window_width, window_height) = window.get_inner_size_pixels().unwrap();
        let (cache_width, cache_height) = (window_width * dpi, window_height * dpi);
        let cache = GlyphCache::new(cache_width, cache_height, 0.1, 0.1);
        let (cache_tex, cache_tex_view) =
            texture::create_texture(&mut factory, window_width, window_height);

        // Put everything together
        let state = GraphicsState {
            factory: factory,
            encoder: encoder,
            options: options,
            objects2d: Vec::new(),
            objects3d: Vec::new(),
            window_size: (width, height),
            window_center: center,
            should_close: false,
            cache: TextCache::new(cache, cache_tex, cache_tex_view),
            pso2d: pso2d,
            pso3d: pso3d,
            pso_gui: pso_gui,
            data3d: data3d,
            data2d: data2d,
            data_gui: data_gui,
            aspect_ratio: aspect_ratio,
            camera: camera,
            assets: Default::default(),
            device: device,
            last_cursor_pos: center,
            pixel_size: (1.0 / width as f32, 1.0 / height as f32),
            dpi: dpi_factor,
        };

        (state, window)
    }
}
