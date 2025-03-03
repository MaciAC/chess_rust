mod app;
mod board;
mod pieces;

use app::AppState;
use board::chess_board::ChessBoard;
use druid::{AppLauncher, WindowDesc, Widget};

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Chess Board")
        .window_size((400.0, 400.0));

    let initial_state = AppState::new();

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_ui() -> impl Widget<AppState> {
    ChessBoard::new()
}
