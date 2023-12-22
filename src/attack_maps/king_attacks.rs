use crate::bitboard::{ZERO, BoardFlat, board_flat};
use crate::attack_maps::pawn_attack::{NOT_A_FILE, NOT_H_FILE};

const fn generate_king_attacks() -> BoardFlat<u64> {
    let mut result = board_flat(0);
    let mut i = 0;

    loop {
        if i == 63 { 
            return result
        } else {
            result[i as usize] = calculate_king_attack_set(i);
        }

        i += 1;
    }
}

const fn calculate_king_attack_set(origin: u64) -> u64 {
    let mut result = ZERO;

    result = result | (origin << 1 & NOT_H_FILE);
    result = result | (origin << 7 & NOT_H_FILE);
    result = result | (origin << 8);
    result = result | (origin << 9);
    result = result | (origin >> 1 & NOT_A_FILE);
    result = result | (origin >> 7);
    result = result | (origin >> 8);
    result = result | (origin >> 9 & NOT_A_FILE);

    result
}

pub const KING_ATTACKS: BoardFlat<u64> = generate_king_attacks();

#[cfg(test)]
mod test {
    use crate::bitboard::{bitboard_from_str, bitboard_to_str};
    use crate::square::Square;

    use super::calculate_king_attack_set;

    #[test]
    fn test_calculate_knigt_attack_set() {
        let attack_on_c4 = bitboard_from_str(
            "........
             ........
             ........
             .xxx....
             .x.x....
             .xxx....
             ........
             ........"
        ).unwrap();   

        let to_test = calculate_king_attack_set(
            Square::C4.as_bitboard()
        );
        println!("{}", bitboard_to_str(Square::C4.as_bitboard()));
        println!("{}", Square::C4.as_bitboard());
        assert_eq!(attack_on_c4, to_test);
    }

}


