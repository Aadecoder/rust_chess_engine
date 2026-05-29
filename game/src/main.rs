pub struct Game{
    board: Board, // Piece Placement
    turn: Color, // black or white
    castling: Option<(Option<Piece>, Option<Piece>, Option<Piece>, Option<Piece>)>, // K Q k q
    en_passant: Option<Square>,
    half_move: u32,
    full_move: u32,
}

pub struct Board{
    squares: [Option<Piece>; 64];
}

pub struct Piece{
    pub color: Color,
    pub kind: Kind,
}

pub enum Color{
    White,
    Black,
}

pub enum Kind{
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Square{
    pub file: usize,
    pub rank: usize,
}

impl Game{
    pub fn new() -> Game{
        Game{
            board: Board::default(),
            turn: Color::White,
            castling: Some((
                    Some(Piece::new(Color::White, Kind::King)),
                    Some(Piece::new(Color::White, Kind::Queen)),
                    Some(Piece::new(Color::Black, Kind::King)),
                    Some(Piece::new(Color::Black, Kind::Queen)),
                    )),
            en_passant: None,
            half_move: 0,
            full_move: 1,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
