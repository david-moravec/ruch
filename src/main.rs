mod bitboard;
mod piece;
mod square;


use crate::bitboard::{Board, fill_board_fen, print_board, DEFAULT_FEN};


fn main() {
    let mut board = Board::new();
    fill_board_fen(&mut board, DEFAULT_FEN);
    print_board(&board);
}
