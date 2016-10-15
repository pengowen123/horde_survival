use conrod::{self, Widget, Colorable, Positionable, Sizeable, widget};

use gui::ids::Ids;

pub fn main_menu_widgets(ui: &mut conrod::UiCell, ids: &Ids) {
    widget::Canvas::new().color(conrod::color::DARK_CHARCOAL).set(ids.canvas, ui);

    // Text to draw
    let demo_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
        Mauris aliquet porttitor tellus vel euismod. Integer lobortis volutpat bibendum. Nulla \
        finibus odio nec elit condimentum, rhoncus fermentum purus lacinia. Interdum et malesuada \
        fames ac ante ipsum primis in faucibus. Cras rhoncus nisi nec dolor bibendum pellentesque. \
        Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. \
        Quisque commodo nibh hendrerit nunc sollicitudin sodales. Cras vitae tempus ipsum. Nam \
        magna est, efficitur suscipit dolor eu, consectetur consectetur urna.";

    widget::Text::new(demo_text)
        .middle_of(ids.canvas)
        .wh_of(ids.canvas)
        .font_size(20)
        .color(conrod::color::BLACK)
        .align_text_middle()
        .set(ids.text, ui);
}
