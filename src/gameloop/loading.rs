use glutin::Window;

use consts::graphics::*;
use hsgraphics::{GraphicsState, shapes3d};
use hsgraphics::object3d::Object3d;
use gameloop::LoopType;
use hslog::CanUnwrap;
use gui::{UI, UIState};

pub fn loading_screen(ui: &mut UI,
                      graphics: &mut GraphicsState,
                      window: &Window,
                      loop_type: &mut LoopType) {

    let name = "loading_screen";
    graphics.assets.add_texture_assets(&[(name, "test_assets/loading_screen.png")]);

    if let Err(e) = graphics.assets.load_texture(name, &mut graphics.factory) {
        crash!(format!("Failed to load texture {}: {}", name, e));
    }

    graphics.encoder.clear(&graphics.data.out_color, GUI_CLEAR_COLOR);

    graphics.draw_gui(window);

    let names = [
        ("floor", "test_assets/floor.png"),
        ("pepe", "test_assets/pepe.png"),
        ("player", "test_assets/player.png"),
        ("zombie", "test_assets/zombie.png"),
        ("blue", "test_assets/blue.png"),
        ("red", "test_assets/red.png"),
        ("black", "test_assets/black.png"),
        ("green", "test_assets/green.png"),
        ("crosshair", "test_assets/crosshair.png"),
        ("ball_linear", "test_assets/ball_linear.png"),
        ("ball_arc", "test_assets/ball_arc.png"),
    ];

    graphics.assets.add_texture_assets(&names);

    for &(name, path) in &names {
        info!("Loading texture '{}' ({})", name, path);
        if let Err(e) = graphics.assets.load_texture(name, &mut graphics.factory) {
            crash!(format!("Failed to load texture {}: {}", name, e));
        }
    }

    // Create floor object, will be removed when maps are added
    let texture = unwrap_or_log!(graphics.assets.get_or_load_texture("floor", &mut graphics.factory),
                                 "Failed to find texture: floor").clone();
    let (v, i) = shapes3d::plane(FLOOR_HEIGHT, 1000.0);
    let floor_object = Object3d::from_slice(&mut graphics.factory, &v, &i, texture);
    graphics.add_object3d(floor_object, 0);

    *loop_type = LoopType::Game;
}
