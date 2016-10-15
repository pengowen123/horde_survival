pub mod ids;
pub mod draw;
pub mod menus;
mod utils;

use conrod::{Ui, UiBuilder};

use hsgraphics::gfx2d::Vertex;

pub struct UI {
    pub ui: Ui,
    pub ids: ids::Ids,
    pub state: UIState,
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
        let mut ui = UiBuilder::new().build();
        let ids = ids::Ids::new(ui.widget_id_generator());

        ui.fonts.insert_from_file("test_assets/Arial Unicode.ttf").unwrap();

        UI {
            ui: ui,
            ids: ids,
            state: UIState::MainMenu,
        }
    }

    pub fn set_widgets(&mut self) {
        let cell = &mut self.ui.set_widgets();
        let ids = &self.ids;
        
        match self.state {
            UIState::MainMenu => self::menus::main_menu_widgets(cell, ids),
            _ => {},
        }
    }
}
