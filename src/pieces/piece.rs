use super::piece_type::PieceType;
use crate::board::chess_board::ChessBoard;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
}

impl Piece {
    /// Gets all theoretically possible moves for the piece without considering board state
    fn get_raw_moves(&self, from: (i32, i32)) -> Vec<(i32, i32)> {
        let mut moves = Vec::new();

        match self.piece_type {
            PieceType::Pawn => {
                let forward = if self.color == PieceColor::White { -1 } else { 1 };
                moves.push((from.0 + forward, from.1));

                // Initial two-square move
                if (from.0 == 6 && self.color == PieceColor::White) ||
                   (from.0 == 1 && self.color == PieceColor::Black) {
                    moves.push((from.0 + forward * 2, from.1));
                }

                // Potential capture moves
                moves.push((from.0 + forward, from.1 - 1));
                moves.push((from.0 + forward, from.1 + 1));
            },
            PieceType::Knight => {
                let knight_moves = [
                    (-2, -1), (-2, 1), (-1, -2), (-1, 2),
                    (1, -2), (1, 2), (2, -1), (2, 1)
                ];
                for &(dx, dy) in &knight_moves {
                    moves.push((from.0 + dx, from.1 + dy));
                }
            },
            PieceType::Bishop => {
                for i in -7..8 {
                    if i == 0 { continue; }
                    moves.push((from.0 + i, from.1 + i));
                    moves.push((from.0 + i, from.1 - i));
                }
            },
            PieceType::Rook => {
                for i in -7..8 {
                    if i == 0 { continue; }
                    moves.push((from.0 + i, from.1));
                    moves.push((from.0, from.1 + i));
                }
            },
            PieceType::Queen => {
                for i in -7..8 {
                    if i == 0 { continue; }
                    // Diagonal moves
                    moves.push((from.0 + i, from.1 + i));
                    moves.push((from.0 + i, from.1 - i));
                    // Straight moves
                    moves.push((from.0 + i, from.1));
                    moves.push((from.0, from.1 + i));
                }
            },
            PieceType::King => {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        moves.push((from.0 + dx, from.1 + dy));
                    }
                }
                // TODO: Add castling moves when implementing that feature
            },
        }

        moves
    }

    /// Gets all valid moves for the piece considering the current board state
    pub fn get_valid_moves(&self, from: (i32, i32), board: &ChessBoard) -> Vec<(i32, i32)> {
        let raw_moves = self.get_raw_moves(from);

        raw_moves.into_iter()
            .filter(|&to| {
                // Check if move is within board bounds
                if to.0 < 0 || to.0 >= 8 || to.1 < 0 || to.1 >= 8 {
                    return false;
                }

                let to_idx = (to.0 * 8 + to.1) as usize;
                let from_idx = (from.0 * 8 + from.1) as usize;

                // Handle pawn special cases
                if self.piece_type == PieceType::Pawn {
                    let dx = (to.1 - from.1).abs();
                    let dy = to.0 - from.0;
                    let forward = if self.color == PieceColor::White { -1 } else { 1 };

                    // Forward moves
                    if dx == 0 {
                        // Single square forward
                        if dy.abs() == 1 {
                            return board.get_piece_at(to_idx).is_none();
                        }
                        // Initial two square move
                        if dy == forward * 2 {
                            let intermediate = (from.0 + forward, from.1);
                            let intermediate_idx = (intermediate.0 * 8 + intermediate.1) as usize;
                            return board.get_piece_at(to_idx).is_none() &&
                                   board.get_piece_at(intermediate_idx).is_none() &&
                                   ((from.0 == 6 && self.color == PieceColor::White) ||
                                    (from.0 == 1 && self.color == PieceColor::Black));
                        }
                        return false;
                    }
                    // Diagonal captures
                    if dx == 1 && dy.abs() == 1 {
                        if let Some(target_piece) = board.get_piece_at(to_idx) {
                            return target_piece.color != self.color;
                        }
                        // TODO: Add en passant when implementing that feature
                        return false;
                    }
                    return false;
                }

                // For all other pieces
                // Check if target square is empty or contains enemy piece
                if let Some(target_piece) = board.get_piece_at(to_idx) {
                    if target_piece.color == self.color {
                        return false;
                    }
                }

                // Knights can jump over pieces
                if self.piece_type == PieceType::Knight {
                    return true;
                }

                // Check if path is clear for other pieces
                board.is_path_clear(from, to)
            })
            .collect()
    }

    pub fn is_valid_move(&self, from: (i32, i32), to: (i32, i32), board: &ChessBoard) -> bool {
        self.get_valid_moves(from, board).contains(&to)
    }
}