use crate::bitboard::{board_flat, BitBoard, BoardFlat};
use crate::constants::{NOT_AB_FILE, NOT_A_FILE, NOT_GH_FILE, NOT_H_FILE, ZERO};

const fn generate_knight_attacks() -> BoardFlat<BitBoard> {
    let mut result = board_flat(0);
    let mut i = 0;

    loop {
        if i == 63 {
            return result;
        } else {
            result[i as usize] = calculate_knight_attack_set(i);
        }

        i += 1;
    }
}

const fn calculate_knight_attack_set(origin: BitBoard) -> BitBoard {
    let mut result = ZERO;

    result = result | (origin << 17 & NOT_A_FILE);
    result = result | (origin << 10 & NOT_AB_FILE);
    result = result | (origin >> 6 & NOT_AB_FILE);
    result = result | (origin >> 15 & NOT_A_FILE);
    result = result | (origin << 15 & NOT_H_FILE);
    result = result | (origin << 6 & NOT_GH_FILE);
    result = result | (origin >> 10 & NOT_GH_FILE);
    result = result | (origin >> 17 & NOT_H_FILE);

    result
}

pub const KNIGHT_ATTACKS: BoardFlat<BitBoard> = generate_knight_attacks();

#[cfg(test)]
mod test {
    use crate::bitboard::bitboard_from_str;
    use crate::square::Square;

    use super::calculate_knight_attack_set;

    #[test]
    fn test_calculate_knigt_attack_set() {
        let attack_on_d4 = bitboard_from_str(
            "........
             ........
             ..x.x...
             .x...x..
             ........
             .x...x..
             ..x.x...
             ........",
        )
        .unwrap();

        let to_test = calculate_knight_attack_set(Square::D4.as_bitboard());
        assert_eq!(attack_on_d4, to_test);
    }
}
