use crate::constants::{NOT_A_FILE, NOT_H_FILE};
use crate::piece::Color;
use crate::types::bitboard::BitBoard;

fn pawn_west_attacks(color: Color, pawn_bitboard: BitBoard) -> BitBoard {
    match color {
        Color::WHITE => pawn_bitboard << 7 & NOT_H_FILE,
        Color::BLACK => pawn_bitboard >> 9 & NOT_H_FILE,
    }
}

fn pawn_east_attacks(color: Color, pawn_bitboard: BitBoard) -> BitBoard {
    match color {
        Color::WHITE => pawn_bitboard << 9 & NOT_A_FILE,
        Color::BLACK => pawn_bitboard >> 7 & NOT_A_FILE,
    }
}

pub fn pawn_any_attacks(color: Color, pawn_bitboard: BitBoard) -> BitBoard {
    pawn_west_attacks(color, pawn_bitboard) | pawn_east_attacks(color, pawn_bitboard)
}

pub fn pawn_double_attacks(color: Color, pawn_bitboard: BitBoard) -> BitBoard {
    pawn_west_attacks(color, pawn_bitboard) & pawn_east_attacks(color, pawn_bitboard)
}

pub fn pawn_single_attacks(color: Color, pawn_bitboard: BitBoard) -> BitBoard {
    pawn_west_attacks(color, pawn_bitboard) ^ pawn_east_attacks(color, pawn_bitboard)
}

#[cfg(test)]
mod test {
    use crate::attack_maps::pawn_attack::pawn_any_attacks;
    use crate::piece::Color::*;
    use crate::types::bitboard::bitboard_from_str;

    #[test]
    fn test_west_pawn_attacks() {
        let start_pos = bitboard_from_str(
            "
            ........
            ........
            ........
            .....x..
            ...x....
            .x......
            ....x...
            ........
        ",
        )
        .unwrap();

        let attacks_black = bitboard_from_str(
            "
            ........
            ........
            ........
            ........
            ....x.x.
            ..x.x...
            x.x.....
            ...x.x..
            
        ",
        )
        .unwrap();
        let attacks = pawn_any_attacks(BLACK, start_pos);
        assert_eq!(attacks_black, attacks);

        let west_attacks_white = bitboard_from_str(
            "
            ........
            ........
            ....x.x.
            ..x.x...
            x.x.....
            ...x.x..
            ........
            ........
            
        ",
        )
        .unwrap();
        let attacks = pawn_any_attacks(WHITE, start_pos);
        assert_eq!(west_attacks_white, attacks);
    }
}
