mod attack_maps;
mod bitboard;
mod piece;
mod square;

use crate::bitboard::{fill_board_fen, print_board, Board, DEFAULT_FEN};

fn main() {
    let mut board = Board::new();
    fill_board_fen(&mut board, DEFAULT_FEN);
    print_board(&board);
}
