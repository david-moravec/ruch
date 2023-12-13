
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Color {
    BLACK,
    WHITE
}

use Color::*;

impl Color {
    pub fn from_char(c: char) -> Self {
        if c.is_uppercase() {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }

    pub fn oposite(self) -> Self {
        if self == Color::WHITE {
            Color::BLACK
        } else {
            Color::WHITE
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Piece {
    PAWN(Color),
    KNIGHT(Color),
    BISHOP(Color),
    ROOK(Color),
    QUEEN(Color),
    KING(Color),
}

use Piece::*;

impl Piece {
    pub fn move_bit_maps() -> u64 {0 as u64}

    pub fn color(&self) -> Color {
        match &self {
            PAWN(WHITE) | KNIGHT(WHITE) | BISHOP(WHITE) | ROOK(WHITE) | QUEEN(WHITE) | KING(WHITE) => WHITE,
            PAWN(BLACK) | KNIGHT(BLACK) | BISHOP(BLACK) | ROOK(BLACK) | QUEEN(BLACK) | KING(BLACK) => BLACK,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePieceError;

impl Piece {
    pub fn to_char(&self) -> char {
           match &self {
               PAWN(WHITE) => 'P',
               KNIGHT(WHITE) => 'N',
               BISHOP(WHITE) => 'B',
               ROOK(WHITE) => 'R',
               QUEEN(WHITE) => 'Q',
               KING(WHITE) => 'K',

               PAWN(BLACK) => 'p',
               KNIGHT(BLACK) => 'n',
               BISHOP(BLACK) => 'b',
               ROOK(BLACK) => 'r',
               QUEEN(BLACK) => 'q',
               KING(BLACK) => 'k',
           } 
    }

    pub fn from_char(c: char) -> Result<Self, ParsePieceError> {
        match c {
           'P' => Ok(PAWN(WHITE)),
           'N' => Ok(KNIGHT(WHITE)),
           'B' => Ok(BISHOP(WHITE)),
           'R' => Ok(ROOK(WHITE)),
           'Q' => Ok(QUEEN(WHITE)),
           'K' => Ok(KING(WHITE)),

           'p' => Ok(PAWN(BLACK)),
           'n' => Ok(KNIGHT(BLACK)),
           'b' => Ok(BISHOP(BLACK)),
           'r' => Ok(ROOK(BLACK)),
           'q' => Ok(QUEEN(BLACK)),
           'k' => Ok(KING(BLACK)),
           _ => Err(ParsePieceError),
       } 
    }
}

pub static PIECE_SET: [Piece; 12] = [
           PAWN(WHITE),
           KNIGHT(WHITE),
           BISHOP(WHITE),
           ROOK(WHITE),
           QUEEN(WHITE),
           KING(WHITE),
           PAWN(BLACK),
           KNIGHT(BLACK),
           BISHOP(BLACK),
           ROOK(BLACK),
           QUEEN(BLACK),
           KING(BLACK),
];

