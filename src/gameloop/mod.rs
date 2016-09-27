pub mod player;
pub mod event;
pub mod game;
pub mod ui;

pub use self::event::handle_event;
pub use self::player::update_player_state;
pub use self::game::gametick;
pub use self::ui::run_gui;

#[derive(Debug)]
pub enum LoopType {
    Game,
    GUI,
}
