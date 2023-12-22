use crate::bitboard::{ZERO, BoardFlat, board_flat};
use crate::attack_maps::pawn_attack::{NOT_A_FILE, NOT_H_FILE};

const NOT_AB_FILE: u64 = NOT_A_FILE & NOT_A_FILE << 1;
const NOT_GH_FILE: u64 = NOT_H_FILE & NOT_H_FILE >> 1;

const fn generate_knight_attacks() -> BoardFlat<u64> {
    let mut result = board_flat(0);
    let mut i = 0;

    loop {
        if i == 63 { 
            return result
        } else {
            result[i as usize] = calculate_knight_attack_set(i);
        }

        i += 1;
    }
}

const fn calculate_knight_attack_set(origin: u64) -> u64 {
    let mut result = ZERO;

    result = result | (origin << 17 & NOT_A_FILE);
    result = result | (origin << 10 & NOT_AB_FILE);
    result = result | (origin >> 6  & NOT_AB_FILE);
    result = result | (origin >> 15 & NOT_A_FILE);
    result = result | (origin << 15 & NOT_H_FILE);
    result = result | (origin << 6  & NOT_GH_FILE);
    result = result | (origin >> 10 & NOT_GH_FILE);
    result = result | (origin >> 17 & NOT_H_FILE);

    result
}

pub const KNIGHT_ATTACKS: BoardFlat<u64> = generate_knight_attacks();

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
             ........"
        ).unwrap();   

        let to_test = calculate_knight_attack_set(
            Square::D4.as_bitboard()
        );
        assert_eq!(attack_on_d4, to_test);
    }

}


