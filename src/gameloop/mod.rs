//! Different gameloops used in different game states
/// Each gameloop function here represents an iteration of the gameloop

pub mod player;
pub mod event;
pub mod game;
pub mod loading;
pub mod ui;

pub use self::event::{handle_event_game, handle_event_gui};
pub use self::player::{update_player_state, update_player_non_tps_bound};
pub use self::game::gametick;
pub use self::ui::run_gui;
pub use self::loading::loading_screen;

/// The mode the game is currently in, controls which gameloop function is used
#[derive(Debug)]
pub enum LoopType {
    Game,
    GUI,
    Loading,
}
