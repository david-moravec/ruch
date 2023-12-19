mod bitboard;
mod piece;
mod square;
mod attack_maps;


use crate::bitboard::{Board, fill_board_fen, print_board, DEFAULT_FEN};
use crate::attack_maps::pawn_attack::test_setup;


fn main() {
    let mut board = Board::new();
    fill_board_fen(&mut board, DEFAULT_FEN);
    print_board(&board);
    let a = test_setup();
}
