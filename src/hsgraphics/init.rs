use glutin::{self, Window, GlRequest};
use gfx::{self, tex, Factory};
use gfx::traits::FactoryExt;
use conrod::text::GlyphCache;
use gfx_window_glutin;

use consts::*;
use hsgraphics::*;
use hsgraphics::object3d::*;
use assets::AssetLoader;
use gamestate::GameState;
use minimap::Minimap;
use hslog::CanUnwrap;
use platform::shaders;

impl GraphicsState {
    pub fn new(options: GraphicsOptions, game: &GameState) -> (GraphicsState, Window) {
        let gl = GlRequest::GlThenGles {
            opengles_version: (2, 0),
            opengl_version: (2, 1),
        };

        let width = options.window_size.0;
        let height = options.window_size.1;
        let center = (width as i32 / 2, height as i32 / 2);

        let mut builder = glutin::WindowBuilder::new()
            .with_title(WINDOW_NAME)
            .with_gl(gl);

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

        let pso2d = match factory.create_pipeline_simple(
            shaders::VERTEX_SHADER_2D,
            shaders::FRAGMENT_SHADER_2D,
            gfx2d::pipe::new()) {
                Ok(p) => p,
                Err(e) => crash!("Failed to create 2d PSO: {}", e),
            };

        let pso3d = match factory.create_pipeline_simple(
            shaders::VERTEX_SHADER_3D,
            shaders::FRAGMENT_SHADER_3D,
            gfx3d::pipe::new()) {
                Ok(p) => p,
                Err(e) => crash!("Failed to create 3d PSO: {}", e),
            };

        let sampler_info = tex::SamplerInfo::new(tex::FilterMethod::Bilinear, tex::WrapMode::Clamp);
        let sampler = factory.create_sampler(sampler_info);
        let aspect_ratio = width as f32 / height as f32;
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

        let dpi_factor = window.hidpi_factor();
        let dpi = dpi_factor as u32;
        let (window_width, window_height) = window.get_inner_size_pixels().unwrap();
        let (cache_width, cache_height) = (window_width * dpi, window_height * dpi);

        let cache = GlyphCache::new(cache_width, cache_height, 0.1, 0.1);
        let (cache_tex, cache_tex_view) = texture::create_cache_texture(&mut factory, window_width, window_height);

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
            data: data,
            data2d: data2d,
            minimap: Minimap::new(MINIMAP_SCALE),
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
