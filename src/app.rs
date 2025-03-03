use druid::{Data, Lens};
use crate::game::GameState;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub selected_square: Option<usize>,
    pub game_state: GameState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            selected_square: None,
            game_state: GameState::new(),
        }
    }
}