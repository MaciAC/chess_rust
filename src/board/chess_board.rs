use druid::{Widget, Color, RenderContext};
use crate::app::AppState;
use crate::pieces::*;
use super::chess_square::ChessSquare;


pub struct ChessBoard {
    squares: Vec<ChessSquare>,
}

impl ChessBoard {
    pub fn new() -> Self {
        let mut squares = Vec::with_capacity(64);
        for row in 0..8 {
            for col in 0..8 {
                let is_light = (row + col) % 2 == 0;
                let piece = match row {
                    0 => Some(Piece {
                        piece_type: match col {
                            0 | 7 => PieceType::Rook,
                            1 | 6 => PieceType::Knight,
                            2 | 5 => PieceType::Bishop,
                            3 => PieceType::Queen,
                            4 => PieceType::King,
                            _ => unreachable!(),
                        },
                        color: PieceColor::Black,
                    }),
                    1 => Some(Piece {
                        piece_type: PieceType::Pawn,
                        color: PieceColor::Black,
                    }),
                    6 => Some(Piece {
                        piece_type: PieceType::Pawn,
                        color: PieceColor::White,
                    }),
                    7 => Some(Piece {
                        piece_type: match col {
                            0 | 7 => PieceType::Rook,
                            1 | 6 => PieceType::Knight,
                            2 | 5 => PieceType::Bishop,
                            3 => PieceType::Queen,
                            4 => PieceType::King,
                            _ => unreachable!(),
                        },
                        color: PieceColor::White,
                    }),
                    _ => None,
                };
                squares.push(ChessSquare::new(is_light, piece));
            }
        }
        Self { squares }
    }

    pub fn get_piece_at(&self, idx: usize) -> Option<Piece> {
        if idx >= 64 {
            return None;
        }
        self.squares[idx].piece
    }

    fn get_possible_moves(&self, square_idx: usize, game_state: &AppState) -> Vec<usize> {
        let _piece = match self.get_piece_at(square_idx) {
            Some(p) => p,
            None => return vec![],
        };

        // Convert squares to board representation for game state
        let mut board = Vec::with_capacity(64);
        for square in &self.squares {
            board.push(square.piece);
        }

        let row = square_idx / 8;
        let col = square_idx % 8;
        let from = (row, col);

        // Get all theoretically valid moves
        let mut valid_moves = Vec::new();
        for to_row in 0..8 {
            for to_col in 0..8 {
                let to = (to_row, to_col);
                if game_state.game_state.is_valid_move(from, to, &board) {
                    valid_moves.push(to_row * 8 + to_col);
                }
            }
        }

        valid_moves
    }

    fn make_move(&mut self, from_idx: usize, to_idx: usize, game_state: &mut AppState) -> bool {
        // Convert squares to board representation for game state
        let mut board = Vec::with_capacity(64);
        for square in &self.squares {
            board.push(square.piece);
        }

        let from = (from_idx / 8, from_idx % 8);
        let to = (to_idx / 8, to_idx % 8);

        if game_state.game_state.make_move(from, to, &mut board) {
            // Update the chess board with the new state
            for (i, piece) in board.into_iter().enumerate() {
                self.squares[i].piece = piece;
            }
            true
        } else {
            false
        }
    }
}

impl Widget<AppState> for ChessBoard {
    fn event(&mut self, ctx: &mut druid::EventCtx, event: &druid::Event, data: &mut AppState, _env: &druid::Env) {
        if let druid::Event::MouseDown(mouse_event) = event {
            let window_size = ctx.window().get_size();
            let width = window_size.width;
            let square_size = width.min(window_size.height) / 8.0;
            let board_width = 8.0 * square_size;
            let x_offset = (width - board_width) / 2.0;

            // Calculate which square was clicked
            let board_x = mouse_event.pos.x - x_offset;
            let board_y = mouse_event.pos.y;

            if board_x >= 0.0 && board_x < board_width && board_y >= 0.0 && board_y < board_width {
                let col = (board_x / square_size) as usize;
                let row = (board_y / square_size) as usize;
                let square_idx = row * 8 + col;

                if let Some(selected) = data.selected_square {
                    if selected == square_idx {
                        // Clicking the same square deselects it
                        data.selected_square = None;
                    } else {
                        // Try to make a move
                        if self.make_move(selected, square_idx, data) {
                            data.selected_square = None;
                        }
                    }
                } else if let Some(piece) = self.squares[square_idx].piece {
                    // Select a piece of the current player's color
                    if piece.color == data.game_state.current_turn {
                        data.selected_square = Some(square_idx);
                    }
                }
                ctx.request_paint();
            }
        }
    }

    fn lifecycle(&mut self, _ctx: &mut druid::LifeCycleCtx, _event: &druid::LifeCycle, _data: &AppState, _env: &druid::Env) {}
    fn update(&mut self, _ctx: &mut druid::UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &druid::Env) {}

    fn layout(&mut self, _ctx: &mut druid::LayoutCtx, bc: &druid::BoxConstraints, _data: &AppState, _env: &druid::Env) -> druid::Size {
        let max_size = bc.max();
        let square_size = max_size.width.min(max_size.height);
        druid::Size::new(square_size, square_size)
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &AppState, _env: &druid::Env) {
        let window_size = ctx.window().get_size();
        let width = window_size.width;
        let square_size = width.min(window_size.height) / 8.0;
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

            // Highlight selected square and possible moves
            let fill_color = if Some(i) == data.selected_square {
                Color::rgb8(255, 255, 0)
            } else if let Some(selected) = data.selected_square {
                if self.get_possible_moves(selected, data).contains(&i) {
                    Color::rgb8(144, 238, 144) // Light green for possible moves
                } else if square.is_light {
                    Color::rgb8(200, 200, 200)
                } else {
                    Color::rgb8(100, 100, 100)
                }
            } else if square.is_light {
                Color::rgb8(200, 200, 200)
            } else {
                Color::rgb8(100, 100, 100)
            };

            ctx.fill(rect, &fill_color);

            // Draw piece if present
            if let Some(piece) = square.piece {
                let piece_color = match piece.color {
                    PieceColor::White => Color::WHITE,
                    PieceColor::Black => Color::BLACK,
                };

                let center_x = x + square_size / 2.0;
                let center_y = y + square_size / 2.0;
                let piece_size = square_size * 0.6;

                match piece.piece_type {
                    PieceType::King => {
                        // Cross base
                        let rect = druid::Rect::from_center_size(
                            (center_x, center_y),
                            (piece_size * 0.2, piece_size),
                        );
                        ctx.fill(rect, &piece_color);
                        let rect = druid::Rect::from_center_size(
                            (center_x, center_y - piece_size * 0.3),
                            (piece_size * 0.6, piece_size * 0.2),
                        );
                        ctx.fill(rect, &piece_color);
                        // Crown circle
                        let circle = druid::kurbo::Circle::new(
                            (center_x, center_y - piece_size * 0.35),
                            piece_size * 0.15,
                        );
                        ctx.fill(circle, &piece_color);
                    },
                    PieceType::Queen => {
                        // Base
                        let mut path = druid::kurbo::BezPath::new();
                        path.move_to((center_x - piece_size * 0.3, center_y + piece_size * 0.3));
                        path.line_to((center_x + piece_size * 0.3, center_y + piece_size * 0.3));
                        path.line_to((center_x, center_y - piece_size * 0.4));
                        path.close_path();
                        ctx.fill(path, &piece_color);
                        // Crown
                        for i in -2..=2 {
                            let circle = druid::kurbo::Circle::new(
                                (center_x + (i as f64) * piece_size * 0.15, center_y - piece_size * 0.25),
                                piece_size * 0.08,
                            );
                            ctx.fill(circle, &piece_color);
                        }
                    },
                    PieceType::Rook => {
                        // Base
                        let rect = druid::Rect::from_center_size(
                            (center_x, center_y + piece_size * 0.1),
                            (piece_size * 0.4, piece_size * 0.6),
                        );
                        ctx.fill(rect, &piece_color);
                        // Battlements
                        for i in -1..=1 {
                            let rect = druid::Rect::from_center_size(
                                (center_x + (i as f64) * piece_size * 0.15, center_y - piece_size * 0.25),
                                (piece_size * 0.1, piece_size * 0.2),
                            );
                            ctx.fill(rect, &piece_color);
                        }
                    },
                    PieceType::Bishop => {
                        // Base triangle
                        let mut path = druid::kurbo::BezPath::new();
                        path.move_to((center_x - piece_size * 0.3, center_y + piece_size * 0.3));
                        path.line_to((center_x + piece_size * 0.3, center_y + piece_size * 0.3));
                        path.line_to((center_x, center_y - piece_size * 0.3));
                        path.close_path();
                        ctx.fill(path, &piece_color);
                        // Top circle
                        let circle = druid::kurbo::Circle::new(
                            (center_x, center_y - piece_size * 0.35),
                            piece_size * 0.1,
                        );
                        ctx.fill(circle, &piece_color);
                    },
                    PieceType::Knight => {
                        // Horse head shape
                        let mut path = druid::kurbo::BezPath::new();
                        path.move_to((center_x - piece_size * 0.2, center_y + piece_size * 0.3));
                        path.line_to((center_x + piece_size * 0.2, center_y + piece_size * 0.3));
                        path.line_to((center_x + piece_size * 0.2, center_y));
                        path.line_to((center_x + piece_size * 0.1, center_y - piece_size * 0.3));
                        path.line_to((center_x - piece_size * 0.2, center_y));
                        path.close_path();
                        ctx.fill(path, &piece_color);
                        // Eye
                        let eye = druid::kurbo::Circle::new(
                            (center_x + piece_size * 0.05, center_y - piece_size * 0.1),
                            piece_size * 0.05,
                        );
                        ctx.fill(eye, &Color::rgb8(50, 50, 50));
                    },
                    PieceType::Pawn => {
                        // Base
                        let circle = druid::kurbo::Circle::new(
                            (center_x, center_y + piece_size * 0.1),
                            piece_size * 0.2,
                        );
                        ctx.fill(circle, &piece_color);
                        // Head
                        let circle = druid::kurbo::Circle::new(
                            (center_x, center_y - piece_size * 0.2),
                            piece_size * 0.15,
                        );
                        ctx.fill(circle, &piece_color);
                    },
                }
            }
        }
    }
}