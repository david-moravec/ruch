use std::collections::HashMap;
use std::iter::{zip, repeat};
use int_enum::IntEnum;
use strum::IntoEnumIterator;

use crate::piece::{Piece, PIECE_SET};
use crate::piece::Color::*;
use crate::square::*;

static ONE: u64 = 1;
static ZERO: u64 = 0;

pub struct Board {
    bit_boards: HashMap<Piece, u64>,
}

fn square_occupied(bboard: u64, square: Square) -> bool {
    bboard & (ONE << square as u64) != 0
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

    fn piece_bit_board(&self, piece: Piece) -> u64 {
        *self.bit_boards.get(&piece).unwrap()
    }

    fn piece_bit_board_mut(&mut self, piece: Piece) -> u64 {
        *self.bit_boards.get_mut(&piece).unwrap()
    }
    
    fn set_piece_bit_board(&mut self, piece: Piece, bitboard: u64) -> () {
        self.bit_boards.insert(piece, bitboard);
    }


    pub fn all_bit_boards(&self) -> u64 {
        let mut result: u64 = 0;

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

    pub fn piecewise_representation(&self) -> [[Option<Piece>; 8]; 8] {
        let mut result: [[Option<Piece>; 8]; 8] = [[None; 8]; 8]; 

        for square in Square::iter() {
            let rank_index = square.rank().unwrap() as usize;
            let file_index = square.file().unwrap() as usize;
            result[file_index][rank_index] = self.piece_on_square(square);
        }
        result
    }
}

fn fill_board_with_pawns(board: &mut Board) -> () {
    let mut black_pawn_bit_board = board.piece_bit_board_mut(Piece::PAWN(BLACK));
    
    for i in Square::B1 as u64 ..= Square::B8 as u64 {
        black_pawn_bit_board = black_pawn_bit_board | (ONE << i);
    }
    print_board(board);
    
    board.set_piece_bit_board(Piece::PAWN(BLACK), black_pawn_bit_board);
}

pub fn fill_board_fen(board: &mut Board, fen_string: &str) -> () {
    let mut i: u64 = 0;

    for c in fen_string.replace("/", "").chars() {
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

pub fn print_board(board: &Board) -> () {
    let piece_bit_board = board.piecewise_representation();
    
    for rank in piece_bit_board {
        for piece_opt in rank{
            match piece_opt {
                Some(piece) => print!("{}", piece.to_char()),
                None => print!(" ")
            }
        } 

        print!("\n")
    }
}

#[cfg(test)]
mod test {
    use super::{Board, fill_board_with_pawns, BLACK, ONE};
    use super::Piece::*;
    use super::Square::*;

    #[test]
    fn test_fill_board_with_pawns() {
        let mut board = Board::new();
        fill_board_with_pawns(&mut board);

        let mut correct_result = 0;

        for i in 8 .. 16 {
            correct_result = correct_result | (ONE << i);
        }

        assert_eq!(board.piece_bit_board(PAWN(BLACK)), correct_result)
    }

    #[test]
    fn test_put_piece_on_square() {
        let mut board = Board::new();
        assert_eq!(board.piece_bit_board(ROOK(BLACK)), 0);
        assert!(board.put_piece_on_square(ROOK(BLACK), C6).is_ok());
        assert_eq!(board.piece_bit_board(ROOK(BLACK)), ONE << C6 as u64);

        assert!(board.put_piece_on_square(ROOK(BLACK), C6).is_err());
    }
}

