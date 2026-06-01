// Each piece would have a color and a kind associated with it
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub pieceKind: PieceKind,
}

// A piece could be either white or black
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

// A piece could be either of these 6 kinds
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Piece {
    pub fn new(color: Color, pieceKind: PieceKind) -> Piece {
        Piece { pieceKind, color }
    }
}
