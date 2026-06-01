// The board representaion would contain information about 64 squares which could or could not
// contain the piece
//
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub struct Board {
    pub white_pawn: u64,
    pub white_kinghts: u64,
    pub white_bishop: u64,
    pub white_rook: u64,
    pub white_queen: u64,
    pub white_king: u64,
    pub black_pawn: u64,
    pub black_kinghts: u64,
    pub black_bishop: u64,
    pub black_rook: u64,
    pub black_queen: u64,
    pub black_king: u64,
}
