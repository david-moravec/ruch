mod bitboard;
mod piece;
mod square;


use crate::bitboard::{Board, fill_board_fen, print_board};

static DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

fn main() {
    let mut board = Board::new();
    fill_board_fen(&mut board, DEFAULT_FEN);
    print_board(&board);
}
