use crate::constants::{NOT_A_FILE, NOT_H_FILE};
use crate::piece::Color;
use crate::types::bitboard::{
    north_east_one, north_west_one, south_east_one, south_west_one, BitBoard,
};

fn pawn_west_attacks(color: Color, pawn_bitboard: BitBoard) -> BitBoard {
    match color {
        Color::WHITE => north_west_one(pawn_bitboard),
        Color::BLACK => south_west_one(pawn_bitboard),
    }
}

fn pawn_east_attacks(color: Color, pawn_bitboard: BitBoard) -> BitBoard {
    match color {
        Color::WHITE => north_east_one(pawn_bitboard),
        Color::BLACK => south_east_one(pawn_bitboard),
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
    use super::*;
    use crate::attack_maps::pawn_attack::pawn_any_attacks;
    use crate::piece::Color::*;
    use crate::types::bitboard::{bitboard_from_str, bitboard_to_str};

    #[test]
    fn test_pawn_west_attacks() {
        let start_pos = bitboard_from_str(
            "
            ........
            ........
            .......x
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
            ......x.
            ....x...
            ..x.....
            .....x..
            
        ",
        )
        .unwrap();
        let attacks = pawn_west_attacks(BLACK, start_pos);
        assert_eq!(attacks_black, attacks);

        let west_attacks_white = bitboard_from_str(
            "
            ........
            ........
            ......x.
            ....x...
            ..x.....
            .....x..
            ........
            ........
            
        ",
        )
        .unwrap();
        let attacks = pawn_west_attacks(WHITE, start_pos);
        assert_eq!(west_attacks_white, attacks);
    }

    #[test]
    fn test_pawn_east_attacks() {
        let start_pos = bitboard_from_str(
            "
            ........
            x.......
            .......x
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
            ......x.
            ....x...
            ..x.....
            x.......
            ...x....
            
        ",
        )
        .unwrap();
        let attacks = pawn_east_attacks(BLACK, start_pos);
        println!(
            "start: \n{}\ncorrect: \n{}\nresult: \n{}\n",
            bitboard_to_str(start_pos),
            bitboard_to_str(attacks_black),
            bitboard_to_str(attacks)
        );
        assert_eq!(attacks_black, attacks);

        let east_attacks_white = bitboard_from_str(
            "
            ........
            ......x.
            ....x...
            ..x.....
            x.......
            ...x....
            ........
            ........
            
        ",
        )
        .unwrap();
        let attacks = pawn_east_attacks(WHITE, start_pos);
        assert_eq!(east_attacks_white, attacks);
    }

    #[test]
    fn test_pawn_attacks() {
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
