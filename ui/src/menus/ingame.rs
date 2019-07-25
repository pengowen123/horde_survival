//! Implementation of the in-game menu (health bar, abilities, etc.)

use common::conrod;
use common::glutin;

use menus::Menus;
use UiState;

impl Menus {
    pub fn set_widgets_in_game(
        &mut self,
        ui: &mut conrod::UiCell,
        ui_state: &mut UiState,
        window: &glutin::Window,
    ) {
    }
}
