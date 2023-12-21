use std::collections::HashMap;
use std::iter::{zip, repeat};
use int_enum::IntEnum;
use strum::IntoEnumIterator;

use crate::piece::{Piece, PIECE_SET};
use crate::square::*;

static ONE: u64 = 1;
pub static ZERO: u64 = 0;
pub static DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

pub struct Board {
    bit_boards: HashMap<Piece, u64>,
}

fn square_occupied(bboard: u64, square: Square) -> bool {
    bboard & (ONE << square as u64) != 0
}

pub type BoardNested<T> = [[T; ROW_COUNT]; FILE_COUNT];
pub fn board_nested<T: Copy>(arg: T) -> BoardNested<T> {
    [[arg; ROW_COUNT]; FILE_COUNT]
}

pub type BoardFlat<T> = [T; SQUARE_COUNT];
pub fn board_flat<T: Copy>(arg: T) -> BoardFlat<T> {
    [arg; SQUARE_COUNT]
}

impl Board {
    pub fn new() -> Board {
        Board { 
            bit_boards: HashMap::from_iter(
                zip(PIECE_SET, repeat(ZERO))
            )
        }
    }

    fn piece_set() -> &'static[Piece] {
        &PIECE_SET
    }

    pub fn piece_bit_board(&self, piece: Piece) -> u64 {
        *self.bit_boards.get(&piece).unwrap()
    }

    fn piece_bit_board_mut(&mut self, piece: Piece) -> u64 {
        *self.bit_boards.get_mut(&piece).unwrap()
    }
    
    pub fn set_piece_bit_board(&mut self, piece: Piece, bitboard: u64) -> () {
        self.bit_boards.insert(piece, bitboard);
    }


    pub fn all_bit_boards(&self) -> u64 {
        let mut result: u64 = ZERO;

        for bitboard in self.bit_boards.values() {
            result = result | bitboard;
        }

        result
    }

    pub fn put_piece_on_square(
        &mut self, piece: Piece, square: Square
    ) -> Result<(), &str> {
        let mut bboard = self.piece_bit_board_mut(piece);

        if square_occupied(bboard, square) {
            return Err("Square occupied")
        } else {
            bboard = bboard | (ONE << square as u64);
            self.set_piece_bit_board(piece, bboard);
            Ok(())
        }
    }

    pub fn piece_on_square(&self, square: Square) -> Option<Piece> {
        for (piece, bitboard) in &self.bit_boards {
            if square_occupied(*bitboard, square){
                return Some(*piece);
            }
        }

        None
    }

    pub fn piecewise_representation(&self) -> BoardNested<Option<Piece>> {
        let mut result_flat = board_flat(None); 
        let mut result = board_nested(None);

        for square in Square::iter() {
            result_flat[square as usize] = self.piece_on_square(square);
        }

        let mut v = Vec::with_capacity(64);

        for chunk in result_flat.chunks(ROW_COUNT) {
            v.extend(chunk.iter().rev());
        }

        for (i, piece_opt) in v.iter().enumerate() {
            let s = Square::try_from(i as u64).unwrap();
            let rank = s.rank().unwrap() as usize;
            let file = s.file().unwrap() as usize;
            result[file][rank] = *piece_opt;
        }

        result.reverse();
        result
    }
}

pub fn fill_board_fen(board: &mut Board, fen_string: &str) -> () {
    let mut i: u64 = 0;

    for c in fen_string.replace("/", "").chars().rev() {
        if c.is_numeric() {
            i += c.to_digit(10).unwrap_or(0) as u64;
        } else {
            board.put_piece_on_square(
                Piece::from_char(c).unwrap(),
                Square::from_int(i).unwrap()
            ).unwrap();
            i += 1;
        }
    }
}

pub fn bitboard_from_str(s: &'static str) -> Result<u64, &'static str> {
    // Used for quickly generating bitboard with a occupancy specified by 'X' or '.'.
    // Inspired by cozy_chess bitboard! macro
    //
    let a = flatten_multiline_string_to_bitboard_repr(s.to_string())?;
    Ok(
        a.iter()
         .enumerate()
         .fold(ZERO, |acc, (i, c)| if *c == 'x' {acc | ONE << i} else {acc})
    )
}

fn flatten_multiline_string_to_bitboard_repr(s: String) -> Result<Vec<char>, &'static str> {
    let mut s = s.to_string();
    s.retain(|c| !c.is_whitespace());
    let s_vec: Vec<char> = s.chars().collect();

    if s_vec.len() != 64 {
        Err("String needs to have one character for each square")
    } else {
        Ok(
            s_vec.chunks(ROW_COUNT)
             .rev()
             .flat_map(|chunk| chunk.iter())
             .map(|c| *c)
             .collect()
        )
    }
}

pub fn bitboard_to_str(bitboard: u64) -> String {
    let mut result_unreversed = board_flat('.');

    for square in Square::iter() {
        if square_occupied(bitboard, square) {
            result_unreversed[square as usize] = 'x';
        }
    } 

    result_unreversed.chunks(ROW_COUNT)
                     .rev()
                     .flat_map(|chunk| chunk.iter().chain(['\n'].iter()))
                     .map(|c| *c)
                     .collect()
}


pub fn print_board(board: &Board) -> () {
    let piece_bit_board = board.piecewise_representation();
    
    print!("  _________________\n");

    for (i, rank) in piece_bit_board.iter().rev().enumerate() {
        print!("{} ", FILE_COUNT - i);

        for piece_opt in rank{
            match piece_opt {
                Some(piece) => print!("|{}", piece.to_char()),
                None => print!("| ")
            }
        } 
        print!("|");
        print!("\n  -----------------\n");
    }

    print!("   ");
    for i in 0..FILE_COUNT {
        let file = File::try_from(i as u64).unwrap();
        print!("{} ", file.to_char());
    }

    print!("\n\n\n")
}

#[cfg(test)]
mod test {
    use crate::piece::Color::{BLACK, WHITE};

    use super::{Board, ONE, fill_board_fen, DEFAULT_FEN, bitboard_from_str, bitboard_to_str, flatten_multiline_string_to_bitboard_repr};
    use super::Piece::*;
    use super::Square::*;

    #[test]
    fn test_put_piece_on_square() {
        let mut board = Board::new();

        assert_eq!(board.piece_bit_board(ROOK(BLACK)), 0);
        assert!(board.put_piece_on_square(ROOK(BLACK), C6).is_ok());

        assert_eq!(board.piece_bit_board(ROOK(BLACK)), ONE << C6 as u64);
        assert!(board.put_piece_on_square(ROOK(BLACK), C6).is_err());
    }

    #[test]
    fn test_fill_board_fen() {
        let mut board = Board::new();
        fill_board_fen(&mut board, DEFAULT_FEN);

        let starting_position = [
            [Some(ROOK(BLACK)), Some(KNIGHT(BLACK)), Some(BISHOP(BLACK)), Some(QUEEN(BLACK)), Some(KING(BLACK)), Some(BISHOP(BLACK)), Some(KNIGHT(BLACK)), Some(ROOK(BLACK))],
            [Some(PAWN(BLACK)), Some(PAWN(BLACK))  , Some(PAWN(BLACK))  , Some(PAWN(BLACK)) , Some(PAWN(BLACK)), Some(PAWN(BLACK))  , Some(PAWN(BLACK)),   Some(PAWN(BLACK))],
            [None,              None,                None,                None,               None,              None,                None,                None],
            [None,              None,                None,                None,               None,              None,                None,                None],
            [None,              None,                None,                None,               None,              None,                None,                None],
            [None,              None,                None,                None,               None,              None,                None,                None],
            [Some(PAWN(WHITE)), Some(PAWN(WHITE))  , Some(PAWN(WHITE))  , Some(PAWN(WHITE)) , Some(PAWN(WHITE)), Some(PAWN(WHITE))  , Some(PAWN(WHITE)),   Some(PAWN(WHITE))],
            [Some(ROOK(WHITE)), Some(KNIGHT(WHITE)), Some(BISHOP(WHITE)), Some(QUEEN(WHITE)), Some(KING(WHITE)), Some(BISHOP(WHITE)), Some(KNIGHT(WHITE)), Some(ROOK(WHITE))],
        ];

        assert_eq!(starting_position, board.piecewise_representation());

        let mut board = Board::new();
        fill_board_fen(&mut board, "8/8/3P2P1/8/PPP5/8/4P1P1/5P2");

        let position = [
            [None,              None,                None,                None,               None,              None,                None,                None],
            [None,              None,                None,                None,               None,              None,                None,                None],
            [None,              None,                None,                Some(PAWN(WHITE)),  None,              None,                Some(PAWN(WHITE)),   None],
            [None,              None,                None,                None,               None,              None,                None,                None],
            [Some(PAWN(WHITE)), Some(PAWN(WHITE)),   Some(PAWN(WHITE)),   None,               None,              None,                None,                None],
            [None,              None,                None,                None,               None,              None,                None,                None],
            [None,              None,                None,                None,               Some(PAWN(WHITE)), None,                Some(PAWN(WHITE)),   None],
            [None,              None,                None,                None,               None,              Some(PAWN(WHITE)),   None,                None],
        ];

        assert_eq!(position, board.piecewise_representation());

    }

    #[test]
    fn test_flatten_multiline_string_to_bitboard_repr() {
        let correct: Vec<char> = Vec::from_iter(
            ['x', '.', '.', '.', 'x', '.', '.', '.',
             'x', '.', '.', '.', '.', 'x', '.', '.',
             '.', '.', '.', '.', '.', '.', 'x', '.',
             '.', '.', '.', '.', '.', '.', '.', '.',
             '.', '.', '.', '.', '.', '.', '.', '.',
             '.', '.', '.', '.', '.', '.', '.', '.',
             '.', '.', '.', '.', '.', '.', '.', '.',
             '.', '.', '.', '.', '.', '.', '.', '.',]
            .iter()
            .map(|c| *c)
        );

        assert_eq!(
            correct,
            flatten_multiline_string_to_bitboard_repr(
                "........
                 ........
                 ........
                 ........
                 ........
                 ......x.
                 x....x..
                 x...x...".to_string()
            ).unwrap()
        ) 

    }

    #[test]
    fn test_bitboard_from_str() {
        let correct = 4202769; 

        assert_eq!(
            correct,
            bitboard_from_str("........
                               ........
                               ........
                               ........
                               ........
                               ......x.
                               x....x..
                               x...x...").unwrap()
        );

        let correct = 44272527353856;
        let string = "........
                      ........
                      ...x.x..
                      ..x...x.
                      ........
                      ..x...x.
                      ...x.x..
                      ........";

        assert_eq!(
            correct,
            bitboard_from_str(string).unwrap()
        ); 

        let correct = 22136263676928;
        let string = "........
                      ........
                      ..x.x...
                      .x...x..
                      ........
                      .x...x..
                      ..x.x...
                      ........";

        assert_eq!(
            correct,
            bitboard_from_str(string).unwrap()
        ) 
    }

    #[test]
    fn test_bitboard_to_str() {
        let n: u64 = 4202769;
        let string = "........
                      ........
                      ........
                      ........
                      ........
                      ......x.
                      x....x..
                      x...x...";

        let mut to_test = bitboard_to_str(n);
        to_test.retain(|c| !c.is_whitespace());  

        let mut correct = string.to_string();
        correct.retain(|c| !c.is_whitespace());

        assert_eq!(correct, to_test);

        let n: u64 = 44272527353856;
        let string = "........
                      ........
                      ...x.x..
                      ..x...x.
                      ........
                      ..x...x.
                      ...x.x..
                      ........";

        let mut to_test = bitboard_to_str(n);
        to_test.retain(|c| !c.is_whitespace());  

        let mut correct = string.to_string();
        correct.retain(|c| !c.is_whitespace());

        assert_eq!(correct, to_test)
    }
}

