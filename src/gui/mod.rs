pub mod text;
pub mod button;
pub mod colors;
pub mod shape;
mod alignment;
mod menus;
mod utils;
mod consts;

pub use self::utils::*;
pub use self::button::*;
pub use self::alignment::*;
pub use self::colors::*;
pub use self::consts::*;
pub use self::shape::*;

use glutin::Window;
use cgmath::Point2;

use self::menus::*;
use hsgraphics::GraphicsState;
use gamestate::GameState;
use gameloop::LoopType;

use std::collections::HashMap;

pub trait UIObject {
    // NOTE: graphics.aspect_ratio stores useful info for drawing fixed size things such as squares
    fn draw(&self, &mut GraphicsState);
    fn is_selected(&self, Point2<f32>) -> bool;

    // This is run when the object is clicked on, and the UI state is set to the return value
    fn select(&mut self, &mut Option<u32>, &UIState, &mut GameState, &mut LoopType, &Window, &mut GraphicsState) -> UIState;
}

pub struct UI {
    pub state: UIState,
    objects: HashMap<u32, Box<UIObject>>,
    selected_id: Option<u32>,
}

#[derive(Clone, PartialEq, Eq)]
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
        println!("Clicked at ({}, {})", mouse.x, mouse.y);

        let mut selected = None;

        for (id, object) in &self.objects {
            if object.is_selected(mouse) {
                selected = Some(*id);
            }
        }

        let state = if let Some(id) = selected {
            let selected_id = &mut self.selected_id;
            let state = &self.state;

            self.objects.get_mut(&id).unwrap().select(selected_id, state, game, loop_type, window, graphics)
        } else {
            return;
        };

        if state != self.state {
            self.set_state(state, graphics);
        }
    }
}
