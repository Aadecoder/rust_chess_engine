use crate::board::Board;
use crate::pieces::{Color, Piece, PieceKind};
use crate::squares::Square;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Game {
    board: Board,
    turn: Color,
    castling: Option<(Option<Piece>, Option<Piece>, Option<Piece>, Option<Piece>)>,
    en_passant: Option<Square>,
    half_move: u32,
    full_move: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::default(),
            turn: Color::White,
            castling: Some((
                Some(Piece::new(Color::White, PieceKind::King)),
                Some(Piece::new(Color::White, PieceKind::Queen)),
                Some(Piece::new(Color::Black, PieceKind::King)),
                Some(Piece::new(Color::Black, PieceKind::Queen)),
            )),
            en_passant: None,
            half_move: 0,
            full_move: 1,
        }
    }
}
