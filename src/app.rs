use druid::Data;
use crate::game::game_state::GameState;

#[derive(Clone, Data)]
pub struct AppState {
    pub game_state: GameState,
    pub selected_square: Option<usize>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            game_state: GameState::new(),
            selected_square: None,
        }
    }
}