use gui::UIState;

pub fn get_button_texture_id(state: UIState, id: u32) -> usize {
    match state {
        UIState::MainMenu => {
            match id {
                0 => 9,
                1 => 9,
                2 => 9,
                _ => 10,
            }
        },
        UIState::ShopMenu => {
            match id {
                0 => 11,
                _ => 10,
            }
        },
        UIState::EscapeMenu => {
            match id {
                0 => 11,
                1 => 9,
                _ => 10,
            }
        },
        UIState::OptionsMenu => {
            match id {
                0 => 9,
                _ => 10,
            }
        },
    }
}
