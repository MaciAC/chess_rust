use druid::widget::{Flex, Label, Container};
use druid::{AppLauncher, Color, Data, Lens, Widget, WindowDesc, RenderContext};

#[derive(Clone, Data, Lens)]
struct AppState {}

#[derive(Clone, Copy)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy)]
enum PieceColor {
    White,
    Black,
}

struct ChessSquare {
    is_light: bool,
    piece: Option<(PieceType, PieceColor)>,
}

struct ChessBoard {
    squares: Vec<ChessSquare>,
}

impl Widget<AppState> for ChessBoard {
    fn event(&mut self, _ctx: &mut druid::EventCtx, _event: &druid::Event, _data: &mut AppState, _env: &druid::Env) {}
    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &AppState, _env: &druid::Env) {}
    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &druid::Env) {}

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, _data: &AppState, _env: &druid::Env) -> druid::Size {
        // Calculate a square size that fits within the constraints
        let max_size = bc.max();
        let square_size = max_size.width.min(max_size.height+30.0);
        druid::Size::new(square_size, square_size)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, _data: &AppState, _env: &druid::Env) {
        let window_size = ctx.window().get_size();
        let width = window_size.width;
        let square_size = width.min(window_size.height-30.0) / 8.0;
        let board_width = 8.0 * square_size;
        let x_offset = (width - board_width) / 2.0;
        for (i, square) in self.squares.iter().enumerate() {
            let row = i / 8;
            let col = i % 8;
            let x = x_offset + col as f64 * square_size;
            let y = row as f64 * square_size;

            let rect = druid::Rect::from_origin_size(
                (x, y),
                (square_size, square_size),
            );
            ctx.fill(rect, &if square.is_light { Color::WHITE } else { Color::BLACK });
        }
    }
}

fn build_ui() -> impl Widget<AppState> {
    let mut board = Flex::column();

    let mut chess_board = ChessBoard { squares: vec![] };
    for row in 0..8 {
        for col in 0..8 {
            let is_light = (row + col) % 2 == 0;
            let piece = match row {
                0 => Some(match col {
                    0 | 7 => (PieceType::Rook, PieceColor::Black),
                    1 | 6 => (PieceType::Knight, PieceColor::Black),
                    2 | 5 => (PieceType::Bishop, PieceColor::Black),
                    3 => (PieceType::Queen, PieceColor::Black),
                    4 => (PieceType::King, PieceColor::Black),
                    _ => unreachable!(),
                }),
                1 => Some((PieceType::Pawn, PieceColor::Black)),
                6 => Some((PieceType::Pawn, PieceColor::White)),
                7 => Some(match col {
                    0 | 7 => (PieceType::Rook, PieceColor::White),
                    1 | 6 => (PieceType::Knight, PieceColor::White),
                    2 | 5 => (PieceType::Bishop, PieceColor::White),
                    3 => (PieceType::Queen, PieceColor::White),
                    4 => (PieceType::King, PieceColor::White),
                    _ => unreachable!(),
                }),
                _ => None,
            };
            chess_board.squares.push(ChessSquare { is_light, piece });
        }
    }
    board.add_child(chess_board);

    board
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .title("Chess Board")
        .window_size((400.0, 400.0));

    let initial_state = AppState {};

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}
