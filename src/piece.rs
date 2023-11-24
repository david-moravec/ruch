use std::fmt;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    BLACK,
    WHITE
}

#[derive(EnumIter, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Piece {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}

impl Piece {
    pub fn move_bit_maps() -> u64 {0 as u64}
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
           match self {
               &Piece::PAWN => write!(f, "p"),
               &Piece::KNIGHT => write!(f, "n"),
               &Piece::BISHOP => write!(f, "b"),
               &Piece::ROOK => write!(f, "r"),
               &Piece::QUEEN => write!(f, "q"),
               &Piece::KING => write!(f, "k"),
           } 
    }
    
}