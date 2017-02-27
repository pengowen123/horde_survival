//! Code for drawing and storing the GUI

pub mod ids;
pub mod draw;
pub mod menus;
pub mod state;
mod crop;
mod utils;

use conrod::{self, Ui, UiBuilder, Theme, color};
use glutin::Window;

use hsgraphics::GraphicsState;
use hsgraphics::texture::Texture;
use gamestate::GameState;
use gameloop::LoopType;
use consts::graphics::GUI_BACKGROUND_COLOR;

/// A wrapper around a conrod Ui that also contains related data
pub struct UI {
    pub ui: Ui,
    pub ids: ids::Ids,
    pub state: UIState,
    pub widget_states: state::WidgetStates,
    pub image_map: conrod::image::Map<Texture>,
}

/// Represents which menu to display
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UIState {
    Main,
    Pause,
    Options,
    Shop,
    NewGame,
    GameOver,
}

impl UI {
    pub fn new() -> UI {
        let theme = Theme {
            background_color: GUI_BACKGROUND_COLOR,
            border_color: color::TRANSPARENT,
            ..Theme::default()
        };

        let mut ui = UiBuilder::new()
            .theme(theme)
            .build();

        let ids = ids::Ids::new(ui.widget_id_generator());

        // Load the font used by horde survival
        // TODO: License stuff for fonts
        ui.fonts.insert_from_file("test_assets/Arial Unicode.ttf").unwrap();

        UI {
            ui: ui,
            ids: ids,
            state: UIState::Main,
            widget_states: Default::default(),
            image_map: conrod::image::Map::new(),
        }
    }

    /// Sets the widgets to use in the GUI
    pub fn set_widgets(&mut self,
                       game: &mut GameState,
                       graphics: &mut GraphicsState,
                       loop_type: &mut LoopType,
                       window: &Window) {

        let cell = &mut self.ui.set_widgets();
        let ids = &self.ids;
        let ui_state = &mut self.state;
        let widget_states = &mut self.widget_states;

        match self.state {
            UIState::Main => {
                menus::main::set_widgets(cell, ids, game, graphics, ui_state, loop_type, window)
            }
            UIState::NewGame => {
                menus::new_game::set_widgets(cell, ids, game, graphics, ui_state, loop_type, window)
            }
            UIState::Shop => {
                menus::shop::set_widgets(cell,
                                         ids,
                                         game,
                                         graphics,
                                         ui_state,
                                         widget_states,
                                         loop_type,
                                         window)
            }
            UIState::Options => {
                menus::options::set_widgets(cell, ids, game, graphics, ui_state, loop_type, window)
            }
            UIState::GameOver => {
                menus::game_over::set_widgets(cell,
                                              ids,
                                              game,
                                              graphics,
                                              ui_state,
                                              loop_type,
                                              window)
            }
            UIState::Pause => {
                menus::pause::set_widgets(cell, ids, game, graphics, ui_state, loop_type, window)
            }
        }
    }
}
