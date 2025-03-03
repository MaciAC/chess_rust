use crate::pieces::{Piece, PieceColor, PieceType};
use druid::Data;
use druid::im::Vector;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Data)]
pub enum GameStatus {
    InProgress,
    Check,
    Checkmate,
    Stalemate,
}

#[derive(Clone, Debug, Data)]
pub struct GameState {
    pub current_turn: PieceColor,
    pub status: GameStatus,
    pub last_move: Option<((usize, usize), (usize, usize))>, // For en passant detection
    pub white_can_castle_kingside: bool,
    pub white_can_castle_queenside: bool,
    pub black_can_castle_kingside: bool,
    pub black_can_castle_queenside: bool,
    pub move_history: Vector<String>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_turn: PieceColor::White,
            status: GameStatus::InProgress,
            last_move: None,
            white_can_castle_kingside: true,
            white_can_castle_queenside: true,
            black_can_castle_kingside: true,
            black_can_castle_queenside: true,
            move_history: Vector::new(),
        }
    }

    pub fn is_valid_move(&self, from: (usize, usize), to: (usize, usize), board: &Vec<Option<Piece>>) -> bool {
        let piece = match board[from.0 * 8 + from.1] {
            Some(p) => p,
            None => return false,
        };

        // Check if it's the correct player's turn
        if piece.color != self.current_turn {
            return false;
        }

        // Convert coordinates for piece movement check
        let from_coords = (from.0 as i32, from.1 as i32);
        let to_coords = (to.0 as i32, to.1 as i32);

        // Get raw moves for the piece
        let raw_moves = piece.get_raw_moves(from_coords);
        if !raw_moves.contains(&to_coords) {
            return false;
        }

        // Special moves check
        if piece.piece_type == PieceType::King {
            // Check if the target square is under attack
            if self.is_square_attacked((to.0, to.1), piece.color, board) {
                return false;
            }

            // Castling
            if self.is_castling_move(from, to, board) {
                return self.is_valid_castling(from, to, board);
            }

            // For regular king moves, check if target square contains friendly piece
            if let Some(target) = board[to.0 * 8 + to.1] {
                if target.color == piece.color {
                    return false;
                }
            }
        } else if piece.piece_type == PieceType::Pawn {
            let dx = (to.1 as i32 - from.1 as i32).abs();
            let dy = to.0 as i32 - from.0 as i32;
            let forward = if piece.color == PieceColor::White { -1 } else { 1 };

            // Check pawn movement rules
            if dx == 0 {
                // Forward moves must be to empty squares
                if board[to.0 * 8 + to.1].is_some() {
                    return false;
                }

                // Two square move requires empty intermediate square
                if dy.abs() == 2 {
                    let intermediate_row = (from.0 as i32 + forward) as usize;
                    if board[intermediate_row * 8 + from.1].is_some() {
                        return false;
                    }
                }
            } else if dx == 1 {
                // Diagonal moves must capture an enemy piece or be en passant
                if !self.is_en_passant_move(from, to, board) {
                    match board[to.0 * 8 + to.1] {
                        Some(target) => {
                            if target.color == piece.color {
                                return false;
                            }
                        }
                        None => return false,
                    }
                }
            }
        } else {
            // For non-pawn pieces, check if target square is empty or contains enemy piece
            if let Some(target) = board[to.0 * 8 + to.1] {
                if target.color == piece.color {
                    return false;
                }
            }

            // Check if path is clear (except for knights)
            if piece.piece_type != PieceType::Knight {
                let dx = to.1 as i32 - from.1 as i32;
                let dy = to.0 as i32 - from.0 as i32;
                let step_x = if dx == 0 { 0 } else { dx / dx.abs() };
                let step_y = if dy == 0 { 0 } else { dy / dy.abs() };

                let mut x = from.1 as i32 + step_x;
                let mut y = from.0 as i32 + step_y;

                while (x, y) != (to.1 as i32, to.0 as i32) {
                    if board[(y as usize) * 8 + (x as usize)].is_some() {
                        return false;
                    }
                    x += step_x;
                    y += step_y;
                }
            }
        }

        // Check if the move would leave the king in check
        if self.would_be_in_check(from, to, board) {
            return false;
        }

        true
    }

    fn is_castling_move(&self, from: (usize, usize), to: (usize, usize), board: &Vec<Option<Piece>>) -> bool {
        let piece = board[from.0 * 8 + from.1].unwrap();
        if piece.piece_type != PieceType::King {
            return false;
        }

        // Check if it's a horizontal move of 2 squares
        from.0 == to.0 && (to.1 as i32 - from.1 as i32).abs() == 2
    }

    fn is_valid_castling(&self, from: (usize, usize), to: (usize, usize), board: &Vec<Option<Piece>>) -> bool {
        let piece = board[from.0 * 8 + from.1].unwrap();

        // Check if king and rook haven't moved
        match (piece.color, to.1) {
            (PieceColor::White, 6) if !self.white_can_castle_kingside => return false,
            (PieceColor::White, 2) if !self.white_can_castle_queenside => return false,
            (PieceColor::Black, 6) if !self.black_can_castle_kingside => return false,
            (PieceColor::Black, 2) if !self.black_can_castle_queenside => return false,
            _ => {}
        }

        // Check if path is clear
        let row = from.0;
        let path_range = if to.1 == 6 { 5..7 } else { 1..4 };

        // Check if squares between king and rook are empty
        for col in path_range {
            if board[row * 8 + col].is_some() {
                return false;
            }
        }

        // Check if king is in check or would pass through check
        let direction = if to.1 == 6 { 1 } else { -1 };
        for col_offset in 0..=2 {
            let check_pos = (row, (from.1 as i32 + col_offset * direction) as usize);
            if self.is_square_attacked(check_pos, piece.color, board) {
                return false;
            }
        }

        true
    }

    fn is_en_passant_move(&self, from: (usize, usize), to: (usize, usize), board: &Vec<Option<Piece>>) -> bool {
        let piece = match board[from.0 * 8 + from.1] {
            Some(p) => p,
            None => return false,
        };

        if piece.piece_type != PieceType::Pawn {
            return false;
        }

        // Check if there was a last move and it was a pawn moving two squares
        if let Some((_, last_to)) = self.last_move {
            let last_piece = board[last_to.0 * 8 + last_to.1].unwrap();
            if last_piece.piece_type == PieceType::Pawn {
                let forward = if piece.color == PieceColor::White { -1 } else { 1 };
                let expected_row = from.0 as i32 + forward;

                // Check if the move is diagonal and captures the pawn that just moved
                if to.0 as i32 == expected_row && (to.1 as i32 - from.1 as i32).abs() == 1 {
                    if last_to.0 == from.0 && last_to.1 == to.1 {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_square_attacked(&self, pos: (usize, usize), defending_color: PieceColor, board: &Vec<Option<Piece>>) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = board[row * 8 + col] {
                    if piece.color != defending_color {
                        let from_coords = (row as i32, col as i32);
                        let to_coords = (pos.0 as i32, pos.1 as i32);

                        // Get raw moves for the attacking piece
                        let raw_moves = piece.get_raw_moves(from_coords);
                        if !raw_moves.contains(&to_coords) {
                            continue;
                        }

                        // For pawns, only consider diagonal attacks
                        if piece.piece_type == PieceType::Pawn {
                            let dx = (to_coords.1 - from_coords.1).abs();
                            let dy = to_coords.0 - from_coords.0;
                            let forward = if piece.color == PieceColor::White { -1 } else { 1 };
                            if dx != 1 || dy != forward {
                                continue;
                            }
                            return true;
                        }

                        // For other pieces, check if path is clear
                        if piece.piece_type == PieceType::Knight {
                            return true;
                        }

                        // Check if path is clear for other pieces
                        let dx = to_coords.1 - from_coords.1;
                        let dy = to_coords.0 - from_coords.0;
                        let step_x = if dx == 0 { 0 } else { dx / dx.abs() };
                        let step_y = if dy == 0 { 0 } else { dy / dy.abs() };

                        let mut x = from_coords.1 + step_x;
                        let mut y = from_coords.0 + step_y;
                        let mut path_clear = true;

                        while (x, y) != (to_coords.1, to_coords.0) {
                            if board[(y as usize) * 8 + (x as usize)].is_some() {
                                path_clear = false;
                                break;
                            }
                            x += step_x;
                            y += step_y;
                        }

                        if path_clear {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn would_be_in_check(&self, from: (usize, usize), to: (usize, usize), board: &Vec<Option<Piece>>) -> bool {
        // Create a temporary board with the move applied
        let mut temp_board = board.clone();
        let moving_piece = temp_board[from.0 * 8 + from.1].take();
        temp_board[to.0 * 8 + to.1] = moving_piece;

        // Find the king's position
        let king_color = moving_piece.unwrap().color;
        let mut king_pos = None;
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = temp_board[row * 8 + col] {
                    if piece.piece_type == PieceType::King && piece.color == king_color {
                        king_pos = Some((row, col));
                        break;
                    }
                }
            }
        }

        if let Some(king_pos) = king_pos {
            self.is_square_attacked(king_pos, king_color, &temp_board)
        } else {
            false
        }
    }

    fn get_square_name(pos: (usize, usize)) -> String {
        let file = (b'a' + pos.1 as u8) as char;
        let rank = 8 - pos.0;
        format!("{}{}", file, rank)
    }

    fn get_piece_symbol(piece: Piece) -> &'static str {
        match piece.piece_type {
            PieceType::King => "K",
            PieceType::Queen => "Q",
            PieceType::Rook => "R",
            PieceType::Bishop => "B",
            PieceType::Knight => "N",
            PieceType::Pawn => "",
        }
    }

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize), board: &mut Vec<Option<Piece>>) -> bool {
        if !self.is_valid_move(from, to, board) {
            return false;
        }

        let piece = board[from.0 * 8 + from.1].unwrap();
        let is_capture = board[to.0 * 8 + to.1].is_some() || self.is_en_passant_move(from, to, board);
        let is_castling = self.is_castling_move(from, to, board);

        // Handle castling
        if is_castling {
            let row = from.0;
            let (rook_from_col, rook_to_col) = if to.1 == 6 { (7, 5) } else { (0, 3) };
            board[row * 8 + rook_to_col] = board[row * 8 + rook_from_col].take();
        }

        // Handle en passant
        if self.is_en_passant_move(from, to, board) {
            board[from.0 * 8 + to.1] = None;
        }

        // Update castling rights
        match piece.piece_type {
            PieceType::King => {
                if piece.color == PieceColor::White {
                    self.white_can_castle_kingside = false;
                    self.white_can_castle_queenside = false;
                } else {
                    self.black_can_castle_kingside = false;
                    self.black_can_castle_queenside = false;
                }
            }
            PieceType::Rook => {
                match (from.0, from.1) {
                    (7, 0) => self.white_can_castle_queenside = false,
                    (7, 7) => self.white_can_castle_kingside = false,
                    (0, 0) => self.black_can_castle_queenside = false,
                    (0, 7) => self.black_can_castle_kingside = false,
                    _ => {}
                }
            }
            _ => {}
        }

        // Make the move
        board[to.0 * 8 + to.1] = board[from.0 * 8 + from.1].take();

        // Record the move in algebraic notation
        let mut move_text = String::new();

        if is_castling {
            move_text = if to.1 == 6 { "O-O".to_string() } else { "O-O-O".to_string() };
        } else {
            move_text.push_str(Self::get_piece_symbol(piece));
            move_text.push_str(&Self::get_square_name(from));
            if is_capture {
                move_text.push('x');
            }
            move_text.push_str(&Self::get_square_name(to));
        }

        // Handle pawn promotion
        if piece.piece_type == PieceType::Pawn {
            if (piece.color == PieceColor::White && to.0 == 0) ||
               (piece.color == PieceColor::Black && to.0 == 7) {
                // Promote to queen by default
                board[to.0 * 8 + to.1] = Some(Piece {
                    piece_type: PieceType::Queen,
                    color: piece.color,
                });
                move_text.push_str("=Q");
            }
        }

        // Update game status
        self.update_game_status(board);

        // Add check or checkmate symbol
        match self.status {
            GameStatus::Check => move_text.push('+'),
            GameStatus::Checkmate => move_text.push('#'),
            _ => {}
        }

        // Add move to history
        if piece.color == PieceColor::White {
            self.move_history.push_back(format!("{}. {}", self.move_history.len() / 2 + 1, move_text));
        } else {
            if let Some(last) = self.move_history.last() {
                let mut new_last = last.clone();
                new_last.push_str(&format!(" {}", move_text));
                self.move_history.pop_back();
                self.move_history.push_back(new_last);
            }
        }

        self.last_move = Some((from, to));

        // Switch turns
        self.current_turn = if self.current_turn == PieceColor::White {
            PieceColor::Black
        } else {
            PieceColor::White
        };

        true
    }

    fn update_game_status(&mut self, board: &Vec<Option<Piece>>) {
        // Find the current player's king
        let mut king_pos = None;
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = board[row * 8 + col] {
                    if piece.piece_type == PieceType::King && piece.color == self.current_turn {
                        king_pos = Some((row, col));
                        break;
                    }
                }
            }
            if king_pos.is_some() {
                break;
            }
        }

        let king_pos = king_pos.unwrap(); // King should always exist

        // Check if the king is under attack
        let in_check = self.is_square_attacked(king_pos, self.current_turn, board);

        if !in_check {
            // If not in check, check for stalemate
            let mut has_legal_moves = false;
            'outer: for from_row in 0..8 {
                for from_col in 0..8 {
                    if let Some(piece) = board[from_row * 8 + from_col] {
                        if piece.color == self.current_turn {
                            let from = (from_row, from_col);
                            // Try all possible moves
                            for to_row in 0..8 {
                                for to_col in 0..8 {
                                    let to = (to_row, to_col);
                                    if self.is_valid_move(from, to, board) {
                                        has_legal_moves = true;
                                        break 'outer;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            self.status = if has_legal_moves {
                GameStatus::InProgress
            } else {
                GameStatus::Stalemate
            };
            return;
        }

        // If in check, look for legal moves to escape check
        let mut has_legal_moves = false;
        'outer: for from_row in 0..8 {
            for from_col in 0..8 {
                if let Some(piece) = board[from_row * 8 + from_col] {
                    if piece.color == self.current_turn {
                        let from = (from_row, from_col);
                        // Try all possible moves
                        for to_row in 0..8 {
                            for to_col in 0..8 {
                                let to = (to_row, to_col);
                                if self.is_valid_move(from, to, board) {
                                    has_legal_moves = true;
                                    break 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }

        self.status = if has_legal_moves {
            GameStatus::Check
        } else {
            GameStatus::Checkmate
        };
    }
}