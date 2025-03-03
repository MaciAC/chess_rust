use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub selected_square: Option<usize>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            selected_square: None,
        }
    }
}