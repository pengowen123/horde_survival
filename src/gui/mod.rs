pub mod text;
pub mod button;
pub mod colors;
pub mod shape;
pub mod picture;
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
pub use self::picture::*;
pub use self::text::*;

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
    fn get_layer(&self) -> usize;
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
    LoadingScreen,
}

impl UI {
    pub fn new() -> UI {
        UI {
            objects: HashMap::new(),
            state: UIState::MainMenu,
            selected_id: None,
        }
    }

    pub fn set_state(&mut self, state: UIState, graphics: &mut GraphicsState) {
        self.objects.clear();
        self.state = state;

        match self.state {
            UIState::MainMenu => get_main_menu_objects(&mut self.objects, graphics),
            UIState::EscapeMenu => get_escape_menu_objects(&mut self.objects, graphics),
            UIState::OptionsMenu => get_options_menu_objects(&mut self.objects, graphics),
            UIState::ShopMenu => get_shop_menu_objects(&mut self.objects, graphics),
            UIState::LoadingScreen => get_loading_screen_objects(&mut self.objects, graphics),
        };
    }
}

impl UI {
    pub fn draw(&self, graphics: &mut GraphicsState) {
        // 0 is the bottom layer, 2 is the top
        for layer in 0..3 {
            for object in self.objects.values().filter(|v| v.get_layer() == layer) {
                object.draw(graphics);
            }
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
