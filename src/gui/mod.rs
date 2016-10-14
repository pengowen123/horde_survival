mod colors;

pub struct UI {
    pub state: UIState,
}

#[derive(Clone, PartialEq, Eq)]
pub enum UIState {
    MainMenu,
    EscapeMenu,
    OptionsMenu,
    ShopMenu,
    LoadingScreen,
}

impl UI {
    pub fn new() -> UI {
        UI {
            state: UIState::MainMenu,
        }
    }
}
