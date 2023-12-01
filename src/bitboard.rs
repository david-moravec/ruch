use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::piece::{Piece, Color};
use crate::piece::Color::{BLACK, WHITE};
use crate::square::Square;

static ONE: u64 = 1;
static ZERO: u64 = 0;

pub struct Board {
   bit_boards: HashMap<Color, HashMap<Piece, u64>>
}

fn square_occupied(bboard: u64, square: Square) -> bool {
    bboard & (ONE << square as u64) == 0
}

impl Board {
    pub fn new() -> Board {
        let one_side_map = HashMap::from([
            (Piece::PAWN, ZERO),
            (Piece::KNIGHT, ZERO),
            (Piece::BISHOP, ZERO),
            (Piece::ROOK, ZERO),
            (Piece::QUEEN, ZERO),
            (Piece::KING, ZERO),
        ]);
        
        let mut board = Board { 
            bit_boards: HashMap::from([
                (WHITE, one_side_map.clone()),
                (BLACK, one_side_map), 
            ])
        };
        
        fill_board_with_pawns(&mut board);
        print_board(&board);
        board
    }

    
    fn pieces_bit_board(&self, piece: Piece) -> u64 {
        self.color_pieces_bit_board(BLACK, piece) 
        | 
        self.color_pieces_bit_board(WHITE, piece)
    }

    fn pieces_bit_board_mut(&mut self, piece: Piece) -> u64 {
        self.color_pieces_bit_board_mut(BLACK, piece) 
        | 
        self.color_pieces_bit_board_mut(WHITE, piece)
    }
    
    fn color_pieces_bit_board(&self, color: Color, piece: Piece) -> u64 {
        *self.bit_boards.get(&color).unwrap()
                        .get(&piece).unwrap()
    }

    fn color_pieces_bit_board_mut(&mut self, color: Color, piece: Piece) -> u64 {
        *self.bit_boards.get_mut(&color).unwrap()
                        .get_mut(&piece).unwrap()
    }
    
    fn set_color_pieces_bit_board(&mut self, color: Color, piece: Piece, bitboard: u64) -> () {
        self.bit_boards.get_mut(&color).unwrap().insert(piece, bitboard);
    }
    
    pub fn put_piece_on_square(
        &mut self, color: Color,piece: Piece, square: Square
    ) -> Result<(), &str> {
        let mut bboard = self.color_pieces_bit_board_mut(color, piece);

        if square_occupied(bboard, square) {
            return Err("Square occupied")
        } else {
            bboard = bboard | (ONE << square as u64);
            self.set_color_pieces_bit_board(color, piece, bboard);

            return Ok(())
        }
    }
}

fn fill_board_with_pawns(board: &mut Board) -> () {
    let mut black_pawn_bit_board = board.color_pieces_bit_board_mut(BLACK, Piece::PAWN);
    
    for i in Square::B1 as u64 ..= Square::B8 as u64 {
        black_pawn_bit_board = black_pawn_bit_board | (ONE << i);
    }
    print_board(board);
    
    board.set_color_pieces_bit_board(BLACK, Piece::PAWN, black_pawn_bit_board);
}

fn print_board(board: &Board) -> () {
    for piece in Piece::iter() {
        let piece_bit_board = board.pieces_bit_board(piece);
        
        for i in 0 .. 64 {
            if i % 8 == 0 && i > 0 {
                print!("\n")
            }
            if (ONE << i & piece_bit_board) != 0 {
                print!("{}", piece)
            } else {
                print!("-")
            }
        }
        print!("\n#####next bit board#####\n")
        
    }
    return
}

#[cfg(test)]
mod test {
    use super::{Board, fill_board_with_pawns, BLACK, Piece, ONE};

    #[test]
    fn test_fill_board_with_pawns() {
        let mut board = Board::new();
        fill_board_with_pawns(&mut board);

        let mut correct_result = 0;

        for i in 8 .. 16 {
            correct_result = correct_result | (ONE << i);
        }

        assert_eq!(board.color_pieces_bit_board(BLACK, Piece::PAWN), correct_result)
    }

}

