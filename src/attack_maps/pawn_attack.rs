use crate::piece::Color;
use crate::bitboard::BitBoard;

pub const NOT_A_FILE: BitBoard = 0xfefefefefefefefe;
pub const NOT_H_FILE: BitBoard = 0x7f7f7f7f7f7f7f7f;

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
    use crate::piece::Color::*;
    use crate::bitboard::bitboard_from_str;
    use super::pawn_west_attacks;

    #[test]
    fn test_west_pawn_attacks() {
        let start_pos = bitboard_from_str("
            ........
            ........
            ........
            .....x..
            ...x....
            .x......
            ....x...
            ........
        ").unwrap();   

        let west_attacks_black = bitboard_from_str("
            ........
            ........
            ........
            ........
            ....x...
            ..x.....
            x.......
            ...x....
            
        ").unwrap();
        let attacks = pawn_west_attacks(BLACK, start_pos);
        assert_eq!(west_attacks_black, attacks);

        let west_attacks_white = bitboard_from_str("
            ........
            ........
            ....x...
            ..x.....
            x.......
            ...x....
            ........
            ........
            
        ").unwrap();
        let attacks = pawn_west_attacks(WHITE, start_pos);
        assert_eq!(west_attacks_white, attacks);

    }
}
