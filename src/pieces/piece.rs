use super::piece_type::PieceType;
use druid::Data;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Data)]
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
    pub fn get_raw_moves(&self, from: (i32, i32)) -> Vec<(i32, i32)> {
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
            },
        }

        moves
    }
}