pub mod ids;
pub mod draw;
pub mod menus;
mod utils;

use conrod::{self, Ui, UiBuilder, Theme, Color, color};

use hsgraphics::gfx2d::Vertex;
use hsgraphics::GraphicsState;
use hsgraphics::texture::Texture;
use gamestate::GameState;
use gameloop::LoopType;

pub struct UI {
    pub ui: Ui,
    pub ids: ids::Ids,
    pub state: UIState,
    pub image_map: conrod::image::Map<Texture>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UIState {
    MainMenu,
    EscapeMenu,
    OptionsMenu,
    ShopMenu,
    LoadingScreen,
}

impl UI {
    pub fn new() -> UI {
        let theme = Theme {
            background_color: color::BLUE,
            .. Theme::default()
        };

        let mut ui = UiBuilder::new()
            .theme(theme)
            .build();

        let ids = ids::Ids::new(ui.widget_id_generator());

        ui.fonts.insert_from_file("test_assets/Arial Unicode.ttf").unwrap();

        UI {
            ui: ui,
            ids: ids,
            state: UIState::MainMenu,
            image_map: conrod::image::Map::new(),
        }
    }

    pub fn set_widgets(&mut self,
                       game: &mut GameState,
                       graphics: &mut GraphicsState,
                       loop_type: &mut LoopType) {

        let cell = &mut self.ui.set_widgets();
        let ids = &self.ids;
        let ui_state = &mut self.state;
        
        match self.state {
            UIState::MainMenu => self::menus::main_menu_widgets(cell,
                                                                ids,
                                                                &self.image_map,
                                                                game,
                                                                graphics,
                                                                ui_state,
                                                                loop_type),
            _ => {},
        }
    }
}
