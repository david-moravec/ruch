use crate::{
    bitboard::BitBoard,
    constants::{A1_H8_DIAG, A_FILE, H_FILE},
};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum RayDirection {
    NORTH,
    NOEAST,
    EAST,
    SOEAST,
    SOUTH,
    SOWEST,
    WEST,
    NOWEST,
}

const fn calculate_south_ray_attack(origin: BitBoard) -> BitBoard {
    let source: BitBoard = H_FILE << 8;
    source << origin.leading_zeros()
}

const fn calculate_north_ray_attack(origin: BitBoard) -> BitBoard {
    let source: BitBoard = A_FILE << 8;
    source << origin.trailing_zeros()
}

fn calculate_ray_attacks_for_each_direction_pos(
    origin: BitBoard,
) -> HashMap<RayDirection, BitBoard> {
    let mut result: HashMap<RayDirection, BitBoard> = HashMap::new();
    let leading_zeros: u32 = origin.leading_zeros();

    result.insert(RayDirection::NORTH, calculate_north_ray_attack(origin));
    result.insert(RayDirection::SOUTH, calculate_south_ray_attack(origin));

    result.insert(RayDirection::NOEAST, {
        let source: BitBoard = A1_H8_DIAG;
        source >> leading_zeros
    });

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bitboard::bitboard_from_str;

    #[test]
    fn test_calculate_ray_attacks_for_each_direction() {
        let d4 = bitboard_from_str(
            "........
             ........
             ........
             ........
             ...x....
             ........
             ........
             ........",
        )
        .unwrap();

        let north_ray_d4 = bitboard_from_str(
            "...x....
             ...x....
             ...x....
             ...x....
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        assert_eq!(north_ray_d4, calculate_north_ray_attack(d4));
    }

    fn test_calculate_south_ray_attack() {
        let south_ray_d4 = bitboard_from_str(
            "........
             ........
             ........
             ........
             ........
             ...x....
             ...x....
             ...x....",
        )
        .unwrap();

        let d4 = bitboard_from_str(
            "........
             ........
             ........
             ........
             ...x....
             ........
             ........
             ........",
        )
        .unwrap();

        assert_eq!(south_ray_d4, calculate_south_ray_attack(d4))
    }
}
