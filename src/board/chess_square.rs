use crate::pieces::*;

pub struct ChessSquare {
    pub is_light: bool,
    pub piece: Option<Piece>,
}

impl ChessSquare {
    pub fn new(is_light: bool, piece: Option<Piece>) -> Self {
        Self { is_light, piece }
    }
}