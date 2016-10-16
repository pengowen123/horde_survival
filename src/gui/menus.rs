// NOTE: depth -2.0 is shown after depth -1.0

use conrod::{
    self,
    Widget,
    Colorable,
    Positionable,
    Sizeable,
    Labelable,
    Borderable,
    widget,
    color,
};

use gui::ids::Ids;
use gui::UIState;
use gamestate::GameState;
use hsgraphics::GraphicsState;
use hsgraphics::texture::Texture;
use gameloop::LoopType;

pub fn main_menu_widgets(ui: &mut conrod::UiCell,
                         ids: &Ids,
                         image_map: &conrod::image::Map<Texture>,
                         game: &mut GameState,
                         graphics: &mut GraphicsState,
                         ui_state: &mut UIState,
                         loop_type: &mut LoopType) {

    let bg_color = ui.theme.background_color;

    // Fullscreen canvas
    widget::Canvas::new()
        .color(bg_color)
        .set(ids.background, ui);

    widget::Button::new()
        .label("foo")
        .label_font_size(80)
        .padded_wh_of(ids.background, 500.0)
        .color(color::DARK_RED)
        .middle_of(ids.background)
        .depth(-3.0)
        .set(ids.button, ui);

    widget::Button::new()
        .label("bar")
        .label_font_size(80)
        .padded_wh_of(ids.background, 500.0)
        .color(color::DARK_RED)
        .middle_of(ids.background)
        .down_from(ids.button, 300.0)
        .depth(-2.0)
        .set(ids.button2, ui);

    widget::Image::new()
        .wh_of(ids.background)
        .middle()
        .depth(-1.0)
        .set(ids.main_menu_image, ui);
}
