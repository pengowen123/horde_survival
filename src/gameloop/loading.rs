//! Gameloop for `LoopType::Loading`

use glutin::Window;

use consts::graphics::*;
use hsgraphics::{GraphicsState, shapes3d};
use hsgraphics::object3d::Object3d;
use gameloop::LoopType;
use gui::UI;

/// Runs the game in Loading mode, meaning assets are being loaded
pub fn loading_screen(ui: &mut UI,
                      graphics: &mut GraphicsState,
                      window: &Window,
                      loop_type: &mut LoopType) {

    // Add the loading screen texture
    let name = "loading_screen";
    graphics.assets.add_texture_assets(&[(name, "test_assets/loading_screen.png")]);

    // Load the loading screen texture
    if let Err(e) = graphics.assets.load_texture(name, &mut graphics.factory) {
        crash!("{}", format!("Failed to load texture {}: {}", name, e));
    }

    // Clear the screen
    graphics.encoder.clear(&graphics.data3d.out_color, LOADING_CLEAR_COLOR);

    // TODO: Draw loading screen image here

    // Draw the GUI (which is just a loading screen)
    graphics.draw_gui(window);

    // A list of names of assets and the paths to those assets
    let names = [("floor", "test_assets/floor.png"),
                 ("pepe", "test_assets/pepe.png"),
                 ("player", "test_assets/player.png"),
                 ("zombie", "test_assets/zombie.png"),
                 ("crosshair", "test_assets/crosshair.png"),
                 ("ball_linear", "test_assets/ball_linear.png"),
                 ("ball_arc", "test_assets/ball_arc.png")];

    // Add all the assets
    graphics.assets.add_texture_assets(&names);

    // Load the assets
    for &(name, path) in &names {
        info!("Loading texture '{}' ({})", name, path);
        if let Err(e) = graphics.assets.load_texture(name, &mut graphics.factory) {
            crash!("{}", format!("Failed to load texture {}: {}", name, e));
        }
    }

    // Create a conrod image map for storing images used in the GUI
    ui.image_map = image_map! {
        (ui.ids.main_menu_image, graphics.assets.get_texture("pepe").unwrap().clone()),
    };

    // Create floor object, will be removed when maps are added
    let texture = graphics.assets
        .get_or_load_texture("floor", &mut graphics.factory)
        .unwrap_or_else(|e| crash!("Failed to find texture: floor ({})", e))
        .clone();
    let (v, i) = shapes3d::plane(FLOOR_HEIGHT, 1000.0);
    let floor_object = Object3d::from_slice(&mut graphics.factory, &v, &i, texture);
    graphics.add_object3d(floor_object, 0);

    // Set the mode to GUI
    *loop_type = LoopType::GUI;
}
