use gui::*;
use hsgraphics::GraphicsState;
use hslog::CanUnwrap;

use std::collections::HashMap;

pub fn get_main_menu_objects(objects: &mut HashMap<u32, Box<UIObject>>, graphics: &mut GraphicsState) {
    let (button0, rect0) = Button::new(0,
                                       rect((0.0, 0.0), (0.4, 0.2)),
                                       graphics,
                                       UIState::MainMenu,
                                       Align::bottom_left().with_offset(0.0, 2.0));
    let (button1, rect1) = Button::new(1,
                                       rect((0.0, 0.0), (0.4, 0.2)),
                                       graphics,
                                       UIState::MainMenu,
                                       Align::bottom_left().with_offset(0.0, 1.0));

    let (button2, rect2) = Button::new(2,
                                       rect((0.0, 0.0), (0.4, 0.2)),
                                       graphics,
                                       UIState::MainMenu,
                                       Align::bottom_left());

    let texture = unwrap_or_log!(graphics.assets.get_or_load_texture("pepe", &mut graphics.factory),
                                 "Failed to load texture: pepe").clone();

    objects.insert(0, uiobject(button0));
    objects.insert(1, uiobject(button1));
    objects.insert(2, uiobject(button2));

    objects.insert(4, uiobject(Text::new_on_button("New game", &rect0, graphics)));
    objects.insert(5, uiobject(Text::new_on_button("Options", &rect1, graphics)));
    objects.insert(6, uiobject(Text::new_on_button("Quit", &rect2, graphics)));
    objects.insert(7, uiobject(Text::new_aligned("Horde Survival", 0.1, Align::top(), graphics)));

    objects.insert(3, uiobject(Picture::new(rect((-1.0, -1.0), (1.0, 1.0)),
                                            graphics,
                                            texture,
                                            Align::center(),
                                            0)));
}

pub fn get_escape_menu_objects(objects: &mut HashMap<u32, Box<UIObject>>, graphics: &mut GraphicsState) {
    let (button0, rect0) = Button::new(0,
                                      rect((0.0, 0.0), (0.4, 0.2)),
                                      graphics,
                                      UIState::EscapeMenu,
                                      Align::bottom_left().with_offset(0.0, 1.0));

    let (button1, rect1) = Button::new(1,
                                       rect((0.0, 0.0), (0.4, 0.2)),
                                       graphics,
                                       UIState::EscapeMenu,
                                       Align::bottom_left());

    objects.insert(0, uiobject(button0));
    objects.insert(1, uiobject(button1));
    objects.insert(2, uiobject(Text::new_on_button("Return to game", &rect0, graphics)));
    objects.insert(3, uiobject(Text::new_on_button("Main menu", &rect1, graphics)));
}

pub fn get_options_menu_objects(objects: &mut HashMap<u32, Box<UIObject>>, graphics: &mut GraphicsState) {
    let (button0, rect0) = Button::new(0,
                                       rect((0.0, 0.0), (0.4, 0.2)),
                                       graphics,
                                       UIState::OptionsMenu,
                                       Align::bottom_right().with_offset(0.5, 1.0));

    objects.insert(0, uiobject(button0));
    objects.insert(1, uiobject(Text::new_on_button("Back", &rect0, graphics)));
}

pub fn get_shop_menu_objects(objects: &mut HashMap<u32, Box<UIObject>>, graphics: &mut GraphicsState) {
    let (button0, rect0) = Button::new(0,
                                       rect((0.0, 0.0), (0.4, 0.2)),
                                       graphics,
                                       UIState::ShopMenu,
                                       Align::bottom_right().with_offset(0.5, 1.0));

    objects.insert(0, uiobject(button0));
    objects.insert(1, uiobject(Text::new_on_button("Start wave", &rect0, graphics)));
}

pub fn get_loading_screen_objects(objects: &mut HashMap<u32, Box<UIObject>>, graphics: &mut GraphicsState) {
    let texture = unwrap_or_log!(graphics.assets.get_or_load_texture("loading_screen", &mut graphics.factory),
                                 "Failed to load texture: loading_screen").clone();

    objects.insert(0, uiobject(Picture::new(rect((-1.0, -1.0), (1.0, 1.0)),
                                            graphics,
                                            texture,
                                            Align::center(),
                                            0)));

    let rect = rect((-0.5, -0.25), (0.0, 0.25));
    objects.insert(1, uiobject(Text::new_on_button("Loading", &rect, graphics)));
}
