pub mod text;
pub mod button;
mod menus;
mod utils;
mod consts;

pub use self::utils::*;
pub use self::button::*;

use glutin::Window;
use cgmath::Point2;

use self::menus::*;
use hsgraphics::GraphicsState;
use gamestate::GameState;
use gameloop::LoopType;

use std::collections::HashMap;

pub trait UIObject {
    fn draw(&self, &mut GraphicsState);
    fn is_selected(&self, mouse: Point2<f32>) -> bool;
    fn select(&mut self,
              selected_id: &mut Option<u32>,
              state: &mut UIState,
              game: &mut GameState,
              loop_type: &mut LoopType,
              window: &Window,
              graphics: &mut GraphicsState);
}

pub struct UI {
    pub state: UIState,
    objects: HashMap<u32, Box<UIObject>>,
    selected_id: Option<u32>,
    can_click: bool,
}

#[derive(Clone)]
pub enum UIState {
    MainMenu,
    EscapeMenu,
    OptionsMenu,
    ShopMenu,
}

impl UI {
    pub fn new(graphics: &mut GraphicsState) -> UI {
        let mut ui = UI {
            objects: HashMap::new(),
            state: UIState::MainMenu,
            selected_id: None,
            can_click: true,
        };

        ui.set_state(UIState::MainMenu, graphics);
        ui
    }

    pub fn set_state(&mut self, state: UIState, graphics: &mut GraphicsState) {
        self.state = state;

        self.objects = match self.state {
            UIState::MainMenu => get_main_menu_objects(graphics),
            UIState::EscapeMenu => get_escape_menu_objects(graphics),
            UIState::OptionsMenu => get_options_menu_objects(graphics),
            UIState::ShopMenu => get_shop_menu_objects(graphics),
        };
    }
}

impl UI {
    pub fn draw(&self, graphics: &mut GraphicsState) {
        for object in self.objects.values() {
            object.draw(graphics);
        }
    }

    pub fn click(&mut self, mouse: Point2<f32>, game: &mut GameState, loop_type: &mut LoopType, window: &Window, graphics: &mut GraphicsState) {
        if self.can_click {
            println!("Clicked at ({}, {})", mouse.x, mouse.y);
            self.can_click = false;

            let mut selected = None;

            for (id, object) in &self.objects {
                if object.is_selected(mouse) {
                    selected = Some(*id);
                }
            }

            if let Some(id) = selected {
                let selected_id = &mut self.selected_id;
                let state = &mut self.state;

                self.objects.get_mut(&id).unwrap().select(selected_id, state, game, loop_type, window, graphics);
            }
        }
    }

    pub fn release_lmb(&mut self) {
        self.can_click = true;
    }
}
