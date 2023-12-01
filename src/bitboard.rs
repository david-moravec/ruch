use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::piece::{Piece, Color};

static ONE: u64 = 1;
static ZERO: u64 = 0;

pub struct Board {
   bit_boards: HashMap<Color, HashMap<Piece, u64>>
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
                (Color::WHITE, one_side_map.clone()),
                (Color::BLACK, one_side_map), 
            ])
        };
        
        fill_board_with_pawns(&mut board);
        print_board(&board);
        board
    }
    
    fn pieces_bit_board(&self, piece: &Piece) -> u64 {
        self.color_pieces_bit_board(&Color::BLACK, piece) 
        | 
        self.color_pieces_bit_board(&Color::WHITE, piece)
    }

    fn pieces_bit_board_mut(&mut self, piece: &Piece) -> u64 {
        self.color_pieces_bit_board_mut(&Color::BLACK, piece) 
        | 
        self.color_pieces_bit_board_mut(&Color::WHITE, piece)
    }
    
    fn color_pieces_bit_board(&self, color: &Color, piece: &Piece) -> u64 {
        *self.bit_boards.get(color).unwrap()
                        .get(piece).unwrap()
    }

    fn color_pieces_bit_board_mut(&mut self, color: &Color, piece: &Piece) -> u64 {
        *self.bit_boards.get_mut(color).unwrap()
                        .get_mut(piece).unwrap()
    }
    
    fn set_color_pieces_bit_board(&mut self, color: &Color, piece: Piece, bitboard: u64) -> () {
        self.bit_boards.get_mut(color).unwrap().insert(piece, bitboard);
    }
    
    fn fill_board(&mut self) -> () {
        fill_board_with_pawns(self);
    }
}

fn fill_board_with_pawns(board: &mut Board) -> () {
    let mut black_pawn_bit_board = board.color_pieces_bit_board_mut(&Color::BLACK, &Piece::PAWN);
    
    for i in 8 .. 16 {
        black_pawn_bit_board = black_pawn_bit_board | (ONE << i);
    }
    
    board.set_color_pieces_bit_board(&Color::BLACK, Piece::PAWN, black_pawn_bit_board);
}

fn print_board(board: &Board) -> () {
    for piece in Piece::iter() {
        let piece_bit_board = board.pieces_bit_board(&piece);
        
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
