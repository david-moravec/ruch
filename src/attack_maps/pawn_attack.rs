use crate::piece::Color;
use crate::bitboard::{Board, fill_board_fen, print_board};
use crate::piece::Color::*;
use crate::piece::Piece::*;

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

fn pawn_west_attacks(color: Color, pawn_bitboard: u64) -> u64 {
    match color {
        Color::WHITE => pawn_bitboard << 9 & NOT_A_FILE,
        Color::BLACK => pawn_bitboard >> 7 & NOT_A_FILE,
    }
}

fn pawn_east_attacks(color: Color, pawn_bitboard: u64) -> u64 {
    match color {
        Color::WHITE => pawn_bitboard << 7 & NOT_H_FILE,
        Color::BLACK => pawn_bitboard >> 9 & NOT_H_FILE,
    }
}

pub fn pawn_any_attacks(color: Color, pawn_bitboard: u64) -> u64 {
    pawn_west_attacks(color, pawn_bitboard) | pawn_east_attacks(color, pawn_bitboard)
}

pub fn pawn_double_attacks(color: Color, pawn_bitboard: u64) -> u64 {
    pawn_west_attacks(color, pawn_bitboard) & pawn_east_attacks(color, pawn_bitboard)
}

pub fn pawn_single_attacks(color: Color, pawn_bitboard: u64) -> u64 {
    pawn_west_attacks(color, pawn_bitboard) ^ pawn_east_attacks(color, pawn_bitboard)
}

pub fn test_setup() -> Board {
    let mut board = Board::new();
    //fill_board_fen(&mut board, "8/8/8/8/1P4P1/P3PP2/2PP3P/8");
    fill_board_fen(&mut board, "8/pp2p3/2p3pp/3p1p2/8/8/8/8");
    print_board(&board);
    let attacks = pawn_west_attacks(BLACK, board.piece_bit_board(PAWN(BLACK)));
    board.set_piece_bit_board(PAWN(BLACK), attacks);
    //let attacks = east_pawn_attacks(BLACK, board.piece_bit_board(PAWN(BLACK)));
    //board.set_piece_bit_board(PAWN(BLACK), attacks);

    print_board(&board);
    board
}

#[cfg(test)]
mod test {
    use crate::piece::Color::*;
    use crate::piece::Piece::*;
    use super::{pawn_west_attacks, pawn_east_attacks, test_setup};

    #[test]
    fn test_west_pawn_attacks() {
        let mut board = test_setup();
        let attacks = pawn_west_attacks(BLACK, board.piece_bit_board(PAWN(BLACK)));
        board.set_piece_bit_board(PAWN(BLACK), attacks)

    }
}
