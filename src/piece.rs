use std::fmt;
use std::str::FromStr;
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePieceError;

impl FromStr for Piece {
    type Err = ParsePieceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
           "p" => Ok(Piece::PAWN),
           "n" => Ok(Piece::KNIGHT),
           "b" => Ok(Piece::BISHOP),
           "r" => Ok(Piece::ROOK),
           "q" => Ok(Piece::QUEEN),
           "k" => Ok(Piece::KING),
           _ => Err(ParsePieceError),
       } 
    }
}

#[cfg(test)]
mod test {
    use super::{Piece, FromStr};
    
    #[test]
    fn test_from_str() {
        assert_eq!(Piece::from_str("p"), Ok(Piece::PAWN));
        assert!(Piece::from_str("x").is_err()); 
    }
}

