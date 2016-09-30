use gui::*;
use hsgraphics::GraphicsState;

use std::collections::HashMap;

pub fn get_main_menu_objects(graphics: &mut GraphicsState) -> HashMap<u32, Box<UIObject>> {
    let mut objects = HashMap::new();

    objects.insert(0, uiobject(Button::new(0,
                                           rect((0.0, 0.0), (0.4, 0.2)),
                                           graphics,
                                           UIState::MainMenu,
                                           Align::center())));

    objects.insert(1, uiobject(Button::new(1,
                                           rect((0.0, 0.0), (0.4, 0.2)),
                                           graphics,
                                           UIState::MainMenu,
                                           Align::center().with_offset(0.0, -1.5))));

    objects.insert(2, uiobject(Button::new(2,
                                           rect((0.0, 0.0), (0.4, 0.2)),
                                           graphics,
                                           UIState::MainMenu,
                                           Align::center().with_offset(0.0, -3.0))));

    objects
}

pub fn get_escape_menu_objects(graphics: &mut GraphicsState) -> HashMap<u32, Box<UIObject>> {
    let mut objects = HashMap::new();

    objects.insert(0, uiobject(Button::new(0,
                                           rect((0.0, 0.0), (0.4, 0.2)),
                                           graphics,
                                           UIState::EscapeMenu,
                                           Align::center())));

    objects.insert(1, uiobject(Button::new(1,
                                           rect((0.0, 0.0), (0.4, 0.2)),
                                           graphics,
                                           UIState::EscapeMenu,
                                           Align::center().with_offset(0.0, -1.5))));
    objects
}

pub fn get_options_menu_objects(graphics: &mut GraphicsState) -> HashMap<u32, Box<UIObject>> {
    let mut objects = HashMap::new();

    objects.insert(0, uiobject(Button::new(0,
                                           rect((0.0, 0.0), (0.4, 0.2)),
                                           graphics,
                                           UIState::OptionsMenu,
                                           Align::bottom_right().with_offset(0.5, 1.0))));
    objects
}

pub fn get_shop_menu_objects(graphics: &mut GraphicsState) -> HashMap<u32, Box<UIObject>> {
    let mut objects = HashMap::new();

    objects.insert(0, uiobject(Button::new(0,
                                           rect((0.0, 0.0), (0.4, 0.2)),
                                           graphics,
                                           UIState::ShopMenu,
                                           Align::bottom_right().with_offset(0.5, 1.0))));

    objects
}
