use int_enum::IntEnum;
use std::collections::HashMap;
use std::iter::{repeat, zip};
use strum::IntoEnumIterator;

use crate::constants::{FILE_COUNT, ONE, RANK_COUNT, SQUARE_COUNT, ZERO};
use crate::piece::{Piece, PIECE_SET};
use crate::types::bitboard::{square_occupied, BitBoard};
use crate::types::file::File;
use crate::types::square::Square;

pub type BoardSerialized<T> = [[T; RANK_COUNT as usize]; FILE_COUNT as usize];
pub const fn board_serialized<T: Copy>(arg: T) -> BoardSerialized<T> {
    [[arg; RANK_COUNT as usize]; FILE_COUNT as usize]
}

fn rotate_serialized_board<T: Copy>(mut board: BoardSerialized<T>) -> BoardSerialized<T> {
    board.reverse();
    board.iter_mut().for_each(|rank| rank.reverse());
    board
}

pub type BoardFlat<T> = [T; SQUARE_COUNT as usize];
pub const fn board_flat<T: Copy>(arg: T) -> BoardFlat<T> {
    [arg; SQUARE_COUNT as usize]
}
pub fn board_flat_non_copy<T, F>(mut cb: F) -> BoardFlat<T>
where
    F: FnMut(usize) -> T,
{
    std::array::from_fn(|i| cb(i))
}

pub struct Board {
    bit_boards: HashMap<Piece, BitBoard>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            bit_boards: HashMap::from_iter(zip(PIECE_SET, repeat(ZERO))),
        }
    }

    fn piece_set() -> &'static [Piece] {
        &PIECE_SET
    }

    pub fn piece_bit_board(&self, piece: Piece) -> BitBoard {
        *self.bit_boards.get(&piece).unwrap()
    }

    fn piece_bit_board_mut(&mut self, piece: Piece) -> BitBoard {
        *self.bit_boards.get_mut(&piece).unwrap()
    }

    pub fn set_piece_bit_board(&mut self, piece: Piece, bitboard: BitBoard) -> () {
        self.bit_boards.insert(piece, bitboard);
    }

    pub fn all_bit_boards(&self) -> BitBoard {
        let mut result: u64 = ZERO;

        for bitboard in self.bit_boards.values() {
            result = result | bitboard;
        }

        result
    }

    pub fn put_piece_on_square(&mut self, piece: Piece, square: Square) -> Result<(), &str> {
        let mut bboard = self.piece_bit_board_mut(piece);

        if square_occupied(bboard, square) {
            eprintln!("Square {:?} occupied", square);
            return Err("Square occupied");
        } else {
            bboard = bboard | (ONE << square as u64);
            self.set_piece_bit_board(piece, bboard);
            Ok(())
        }
    }

    pub fn piece_on_square(&self, square: Square) -> Option<Piece> {
        for (piece, bitboard) in &self.bit_boards {
            if square_occupied(*bitboard, square) {
                return Some(*piece);
            }
        }

        None
    }

    pub fn piecewise_representation(&self) -> BoardSerialized<Option<Piece>> {
        let mut result_flat = board_flat(None);
        let mut result = board_serialized(None);

        for square in Square::iter() {
            result_flat[square as usize] = self.piece_on_square(square);
        }

        for (i, piece_opt) in result_flat.iter().enumerate() {
            let s = Square::try_from(i as u64).unwrap();
            let rank = s.rank().unwrap() as usize;
            let file = s.file().unwrap() as usize;
            result[rank][file] = *piece_opt;
        }

        result
    }
}

pub fn fill_board_fen(board: &mut Board, fen_string: &str) -> () {
    let mut i: u64 = 0;

    for c in fen_string.replace("/", "").chars().rev() {
        if c.is_numeric() {
            i += c.to_digit(10).unwrap_or(0) as u64;
        } else {
            board
                .put_piece_on_square(Piece::from_char(c).unwrap(), Square::from_int(i).unwrap())
                .unwrap();
            i += 1;
        }
    }
}

pub fn print_board(board: &Board) -> () {
    let piece_bit_board = board.piecewise_representation();

    print!("  _________________\n");

    for (i, rank) in piece_bit_board.iter().rev().enumerate() {
        print!("{} ", FILE_COUNT as usize - i);

        for piece_opt in rank {
            match piece_opt {
                Some(piece) => print!("|{}", piece.to_char()),
                None => print!("| "),
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
    use crate::constants::{DEFAULT_FEN, ONE};
    use crate::piece::Color::{BLACK, WHITE};
    use crate::piece::Piece;

    use super::Piece::*;
    use super::Square::*;
    use super::*;

    #[test]
    fn test_rotate_serialized_board() {
        let mut unrotated = board_serialized(0);
        unrotated[0][0] = 1; // 1.....2
        unrotated[0][7] = 2; // .......
        unrotated[7][0] = 3; // .......
        unrotated[7][7] = 4; // 3.....4

        let rotated = rotate_serialized_board(unrotated.clone());
        // 4.....3
        // .......
        // .......
        // 2.....1

        assert_eq!(unrotated[0][0], rotated[7][7]);
        assert_eq!(unrotated[0][7], rotated[7][0]);
        assert_eq!(unrotated[7][0], rotated[0][7]);
        assert_eq!(unrotated[7][7], rotated[0][0]);
    }

    #[test]
    fn test_put_piece_on_square() {
        let mut board = Board::new();

        assert_eq!(board.piece_bit_board(ROOK(BLACK)), 0);
        assert!(board.put_piece_on_square(ROOK(BLACK), C6).is_ok());

        assert_eq!(board.piece_bit_board(ROOK(BLACK)), ONE << C6 as u64);
        assert!(board.put_piece_on_square(ROOK(BLACK), C6).is_err());
    }

    #[test]
    fn test_piecewise_representation() {
        let mut board = Board::new();
        let a1_b2_c3 = 262657; // A1, B2, C3
        board.set_piece_bit_board(PAWN(BLACK), a1_b2_c3);
        let f3_g2_h1 = 2113664; // F3, G2, H1
        board.set_piece_bit_board(PAWN(WHITE), f3_g2_h1);

        let mut position: BoardSerialized<Option<Piece>> = board_serialized(None);
        position[0][0] = Some(PAWN(BLACK)); // A1
        position[1][1] = Some(PAWN(BLACK)); // B2
        position[2][2] = Some(PAWN(BLACK)); // C3
        position[2][5] = Some(PAWN(WHITE)); // G3
        position[1][6] = Some(PAWN(WHITE)); // F2
        position[0][7] = Some(PAWN(WHITE)); // H1

        assert_eq!(position, board.piecewise_representation());
    }

    #[test]
    fn test_fill_board_fen() {
        let mut board = Board::new();
        fill_board_fen(&mut board, DEFAULT_FEN);

        let starting_position = [
            [
                Some(ROOK(BLACK)),
                Some(KNIGHT(BLACK)),
                Some(BISHOP(BLACK)),
                Some(QUEEN(BLACK)),
                Some(KING(BLACK)),
                Some(BISHOP(BLACK)),
                Some(KNIGHT(BLACK)),
                Some(ROOK(BLACK)),
            ],
            [
                Some(PAWN(BLACK)),
                Some(PAWN(BLACK)),
                Some(PAWN(BLACK)),
                Some(PAWN(BLACK)),
                Some(PAWN(BLACK)),
                Some(PAWN(BLACK)),
                Some(PAWN(BLACK)),
                Some(PAWN(BLACK)),
            ],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [None, None, None, None, None, None, None, None],
            [
                Some(PAWN(WHITE)),
                Some(PAWN(WHITE)),
                Some(PAWN(WHITE)),
                Some(PAWN(WHITE)),
                Some(PAWN(WHITE)),
                Some(PAWN(WHITE)),
                Some(PAWN(WHITE)),
                Some(PAWN(WHITE)),
            ],
            [
                Some(ROOK(WHITE)),
                Some(KNIGHT(WHITE)),
                Some(BISHOP(WHITE)),
                Some(QUEEN(WHITE)),
                Some(KING(WHITE)),
                Some(BISHOP(WHITE)),
                Some(KNIGHT(WHITE)),
                Some(ROOK(WHITE)),
            ],
        ];

        assert_eq!(
            rotate_serialized_board(starting_position),
            board.piecewise_representation()
        );
    }
}
