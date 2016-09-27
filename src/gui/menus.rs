use gui::*;
use hsgraphics::GraphicsState;

use std::collections::HashMap;

pub fn get_main_menu_objects(graphics: &mut GraphicsState) -> HashMap<u32, Box<UIObject>> {
    let mut objects = HashMap::new();

    objects.insert(0, uiobject(Button::new(0,
                                           rect((-0.2, -0.1), (0.2, 0.1)),
                                           graphics)));

    objects
}

pub fn get_escape_menu_objects(graphics: &mut GraphicsState) -> HashMap<u32, Box<UIObject>> {
    let mut objects = HashMap::new();

    objects
}

pub fn get_options_menu_objects(graphics: &mut GraphicsState) -> HashMap<u32, Box<UIObject>> {
    let mut objects = HashMap::new();

    objects
}

pub fn get_shop_menu_objects(graphics: &mut GraphicsState) -> HashMap<u32, Box<UIObject>> {
    let mut objects = HashMap::new();

    objects.insert(0, uiobject(Button::new(0,
                                           rect((0.4, -0.6), (0.7, -0.4)),
                                           graphics)));

    objects
}
