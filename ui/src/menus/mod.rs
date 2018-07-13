//! Implementation of game menus

mod main;
mod ingame;
mod pause;
mod options;

use common::conrod::widget::id;

widget_ids! {
    pub struct Ids {
        // Main menu
        main_canvas,
        title_text,
        exit_button,
        start_game_button,
        // Pause menu
        pause_canvas,
        resume_game_button,
        exit_to_main_menu_button,
    }
}

/// Stores the state required to run each menu
pub struct Menus {
    ids: Ids,
}

impl Menus {
    pub fn new(ui: id::Generator) -> Self {
        Menus {
            ids: Ids::new(ui),
        }
    }
}
