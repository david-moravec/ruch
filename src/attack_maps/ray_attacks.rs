use crate::{
    constants::{A1_H8_DIAG, A_FILE, EIGHT_RANK, H_FILE, ONE_RANK},
    types::bitboard::BitBoard,
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
    let source: BitBoard = H_FILE >> 8;
    source >> origin.leading_zeros()
}

const fn calculate_north_ray_attack(origin: BitBoard) -> BitBoard {
    let source: BitBoard = A_FILE << 8;
    source << origin.trailing_zeros()
}

const fn calculate_east_ray_attack(origin: BitBoard) -> BitBoard {
    let source: BitBoard = EIGHT_RANK >> 1;
    let not_rank_below = !(EIGHT_RANK >> 8 * (origin.leading_zeros() / 8 + 1));

    source >> origin.leading_zeros() & not_rank_below
}

const fn calculate_west_ray_attack(origin: BitBoard) -> BitBoard {
    let source: BitBoard = ONE_RANK << 1;
    let not_rank_above = !(ONE_RANK << 8 * (origin.trailing_zeros() / 8 + 1));

    source << origin.trailing_zeros() & not_rank_above
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
    use crate::types::bitboard::{bitboard_from_str, bitboard_to_str};

    #[test]
    fn test_calculate_north_ray_attacks() {
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

    #[test]
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

    #[test]
    fn test_calculate_west_ray_attack() {
        let east_ray_d4 = bitboard_from_str(
            "........
             ........
             ........
             ........
             ....xxxx
             ........
             ........
             ........",
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

        println!("{}", bitboard_to_str(calculate_west_ray_attack(d4)));

        assert_eq!(east_ray_d4, calculate_west_ray_attack(d4))
    }

    #[test]
    fn test_calculate_east_ray_attack() {
        let east_ray_d4 = bitboard_from_str(
            "........
             ........
             ........
             ........
             xxx.....
             ........
             ........
             ........",
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

        let ray_attack = calculate_east_ray_attack(d4);

        println!("ray attack \n{}", bitboard_to_str(ray_attack));

        assert_eq!(east_ray_d4, ray_attack)
    }
}
