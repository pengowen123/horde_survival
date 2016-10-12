use gui::UIState;

#[allow(match_same_arms)]
pub fn get_button_texture_name(state: UIState, id: u32) -> &'static str {
    match state {
        UIState::MainMenu => {
            match id {
                0 => "blue",
                1 => "blue",
                2 => "blue",
                _ => "black",
            }
        },
        UIState::ShopMenu => {
            match id {
                0 => "green",
                _ => "black",
            }
        },
        UIState::EscapeMenu => {
            match id {
                0 => "green",
                1 => "blue",
                _ => "black",
            }
        },
        UIState::OptionsMenu => {
            match id {
                0 => "blue",
                _ => "black",
            }
        },
        UIState::LoadingScreen => "black",
    }
}
