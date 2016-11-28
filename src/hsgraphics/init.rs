use glutin::{self, Window};
use gfx::{self, Factory, Primitive, tex};
use gfx::traits::FactoryExt;
use conrod::text::GlyphCache;
use gfx_window_glutin;

use consts::*;
use hsgraphics::*;
use hsgraphics::shaders::*;
use assets::AssetLoader;
use gamestate::GameState;

impl GraphicsState {
    pub fn new(options: GraphicsOptions, game: &GameState) -> (GraphicsState, Window) {
        let width = options.window_size.0;
        let height = options.window_size.1;
        let center = (width as i32 / 2, height as i32 / 2);

        let mut builder = glutin::WindowBuilder::new()
            .with_title(WINDOW_NAME);

        if options.fullscreen {
            let monitor = glutin::get_primary_monitor();
            let (width, height) = monitor.get_dimensions();
            builder = builder
                .with_fullscreen(monitor)
                .with_dimensions(width, height);
        } else {
            builder = builder.with_dimensions(width, height);
        }

        let (window, device, mut factory, main_color, main_depth) =
            gfx_window_glutin::init::<ColorFormat, gfx3d::DepthFormat>(builder);
        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        info!("Platform name: {:?}", device.get_info().platform_name);
        info!("GL version: {:?}", device.get_info().version);
        info!("GLSL version: {:?}", device.get_info().shading_language);

        let shader_assets_path = "test_assets/shaders";
        let vs_2d_path = get_shader_version_path(&device, shader_assets_path, VS_2D_PATH);
        let vs_3d_path = get_shader_version_path(&device, shader_assets_path, VS_3D_PATH);
        let vs_gui_path = get_shader_version_path(&device, shader_assets_path, VS_GUI_PATH);
        let fs_2d_path = get_shader_version_path(&device, shader_assets_path, FS_2D_PATH);
        let fs_3d_path = get_shader_version_path(&device, shader_assets_path, FS_3D_PATH);
        let fs_gui_path = get_shader_version_path(&device, shader_assets_path, FS_GUI_PATH);

        log_create_pso!("2d", VS_2D_PATH, FS_2D_PATH);
        let pso2d = unwrap_pretty!(load_pso(&mut factory, vs_2d_path, fs_2d_path, Primitive::TriangleList, gfx2d::pipe::new()));

        log_create_pso!("3d", VS_3D_PATH, FS_3D_PATH);
        let pso3d = unwrap_pretty!(load_pso(&mut factory, vs_3d_path, fs_3d_path, Primitive::TriangleList, gfx3d::pipe::new()));

        log_create_pso!("gui", VS_GUI_PATH, FS_GUI_PATH);
        let pso_gui = unwrap_pretty!(load_pso(&mut factory, vs_gui_path, fs_gui_path, Primitive::TriangleList, gfx_gui::pipe::new()));

        let sampler_info = tex::SamplerInfo::new(tex::FilterMethod::Bilinear, tex::WrapMode::Clamp);
        let sampler = factory.create_sampler(sampler_info);
        let aspect_ratio = width as f32 / height as f32;
        let camera = get_camera(game.map.player_spawn.clone(), START_CAMERA_ANGLE, aspect_ratio);

        let vbuf = factory.create_vertex_buffer(&[]);
        let vbuf2d = factory.create_vertex_buffer(&[]);
        let vbuf_gui = factory.create_vertex_buffer(&[]);
        let texture = load_texture_raw(&mut factory, [2, 2], &[0; 4]);
        
        let data = gfx3d::pipe::Data {
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

        let dpi_factor = window.hidpi_factor();
        let dpi = dpi_factor as u32;
        let (window_width, window_height) = window.get_inner_size_pixels().unwrap();
        let (cache_width, cache_height) = (window_width * dpi, window_height * dpi);

        let cache = GlyphCache::new(cache_width, cache_height, 0.1, 0.1);
        let (cache_tex, cache_tex_view) = texture::create_cache_texture(
            &mut factory,
            window_width,
            window_height
        );

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
            data: data,
            data2d: data2d,
            data_gui: data_gui,
            aspect_ratio: aspect_ratio,
            camera: camera,
            assets: AssetLoader::new(),
            device: device,
            last_cursor_pos: center,
            pixel_size: (1.0 / width as f32, 1.0 / height as f32),
            dpi: dpi_factor,
        };

        (state, window)
    }
}
