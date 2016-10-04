use glutin::Window;

use consts::graphics::GUI_CLEAR_COLOR;
use hsgraphics::GraphicsState;
use gameloop::LoopType;
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

    ui.set_state(UIState::LoadingScreen, graphics);

    graphics.encoder.clear(&graphics.data.out_color, GUI_CLEAR_COLOR);
    ui.draw(graphics);
    graphics.draw_gui(window);

    let names = [
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

    for &(name, path) in names.iter() {
        info!("Loading texture '{}' ({})", name, path);
        if let Err(e) = graphics.assets.load_texture(name, &mut graphics.factory) {
            crash!(format!("Failed to load texture {}: {}", name, e));
        }
    }

    ui.set_state(UIState::MainMenu, graphics);

    *loop_type = LoopType::GUI;
}
