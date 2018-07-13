//! Common resources

/// Represents which state the UI is in
///
/// If the state is `InGame`, the in-game menu is drawn and all other systems are run. Otherwise,
/// only the UI system is run.
// NOTE: This must be in the `common` crate to avoid cyclic dependencies
#[derive(Clone, Copy, Debug)]
pub enum UiState {
    MainMenu,
    /// The in-game menu (normally includes displays for info like health, ability cooldowns, etc.)
    InGame,
    /// The menu that is displayed while the game is paused
    PauseMenu,
    /// The options menu
    OptionsMenu,
    /// This UI state is used to signal that the game should close
    Exit,
}

impl UiState {
    /// Returns whether other game systems should be run while in this UI state
    pub fn is_in_game(&self) -> bool {
        if let UiState::InGame = *self {
            true
        } else {
            false
        }
    }

    /// Returns whether to close the game
    pub fn should_exit(&self) -> bool {
        if let UiState::Exit = *self {
            true
        } else {
            false
        }
    }
}
