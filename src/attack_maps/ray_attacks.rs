use crate::{
    board::{board_flat, board_flat_non_copy, BoardFlat},
    constants::{A1_H8_DIAG, A_FILE, EIGHT_RANK, H1_A8_DIAG, H_FILE, ONE_RANK},
    types::bitboard::{
        east_one, north_east_one, north_one, north_west_one, south_east_one, south_one,
        south_west_one, west_one, BitBoard,
    },
    types::square::Square,
};

use std::collections::HashMap;
use strum::IntoEnumIterator;

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
    south_one(H_FILE) >> origin.leading_zeros()
}

const fn calculate_north_ray_attack(origin: BitBoard) -> BitBoard {
    north_one(A_FILE) << origin.trailing_zeros()
}

const fn calculate_east_ray_attack(origin: BitBoard) -> BitBoard {
    let source: BitBoard = east_one(EIGHT_RANK);
    let not_rank_below = !(EIGHT_RANK >> 8 * (origin.leading_zeros() / 8 + 1));

    source >> origin.leading_zeros() & not_rank_below
}

const fn calculate_west_ray_attack(origin: BitBoard) -> BitBoard {
    let source: BitBoard = west_one(ONE_RANK);
    let not_rank_above = !(ONE_RANK << 8 * (origin.trailing_zeros() / 8 + 1));

    source << origin.trailing_zeros() & not_rank_above
}

const fn calculate_nowest_ray_attack(origin: BitBoard) -> BitBoard {
    let mut result: BitBoard = north_west_one(A1_H8_DIAG);

    let trailing_zeros = origin.trailing_zeros();
    let mut i = 0;

    let file_count = trailing_zeros % 8;
    let rank_count = trailing_zeros / 8;

    while i < file_count {
        result = west_one(result);
        i += 1;
    }

    result << rank_count * 8
}

const fn calculate_soeast_ray_attack(origin: BitBoard) -> BitBoard {
    let mut result: BitBoard = south_east_one(A1_H8_DIAG);

    let leading_zeros = origin.leading_zeros();
    let mut i = 0;

    let file_count = leading_zeros % 8;
    let rank_count = leading_zeros / 8;

    while i < file_count {
        result = east_one(result);
        i += 1;
    }

    result >> rank_count * 8
}

const fn calculate_noeast_ray_attack(origin: BitBoard) -> BitBoard {
    let mut result: BitBoard = north_east_one(H1_A8_DIAG);

    let mut i = 0;

    let file_count = origin.leading_zeros() % 8;
    let rank_count = origin.trailing_zeros() / 8;

    while i < file_count {
        result = east_one(result);
        i += 1;
    }

    result << rank_count * 8
}

const fn calculate_sowest_ray_attack(origin: BitBoard) -> BitBoard {
    let mut result: BitBoard = south_west_one(H1_A8_DIAG);

    let mut i = 0;

    let file_count = origin.trailing_zeros() % 8;
    let rank_count = origin.leading_zeros() / 8;

    while i < file_count {
        result = west_one(result);
        i += 1;
    }

    result >> rank_count * 8
}

type RayCollection = HashMap<RayDirection, BitBoard>;

fn ray_collection(origin: usize) -> RayCollection {
    let origin_bitboard: BitBoard = Square::try_from(origin as u64)
        .expect("Cannot convert number to square")
        .as_bitboard();

    HashMap::from([
        (
            RayDirection::NORTH,
            calculate_north_ray_attack(origin_bitboard),
        ),
        (
            RayDirection::SOUTH,
            calculate_south_ray_attack(origin_bitboard),
        ),
        (
            RayDirection::WEST,
            calculate_west_ray_attack(origin_bitboard),
        ),
        (
            RayDirection::EAST,
            calculate_east_ray_attack(origin_bitboard),
        ),
        (
            RayDirection::NOWEST,
            calculate_nowest_ray_attack(origin_bitboard),
        ),
        (
            RayDirection::NOEAST,
            calculate_noeast_ray_attack(origin_bitboard),
        ),
        (
            RayDirection::SOWEST,
            calculate_sowest_ray_attack(origin_bitboard),
        ),
        (
            RayDirection::SOEAST,
            calculate_soeast_ray_attack(origin_bitboard),
        ),
    ])
}

fn calculate_ray_collection_for_each_square() -> BoardFlat<RayCollection> {
    board_flat_non_copy(ray_collection)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{constants::SQUARE_COUNT, types::bitboard::bitboard_from_str};

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

        assert_eq!(east_ray_d4, ray_attack)
    }

    #[test]
    fn test_calculate_north_west_ray_attack() {
        let correct_ray_attack = bitboard_from_str(
            "....x...
             ...x....
             ........
             ........
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let d4 = bitboard_from_str(
            "........
             ........
             ..x.....
             ........
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let calculated_ray_attack = calculate_nowest_ray_attack(d4);

        assert_eq!(correct_ray_attack, calculated_ray_attack)
    }

    #[test]
    fn test_calculate_south_east_ray_attack() {
        let correct_ray_attack = bitboard_from_str(
            "........
             ........
             ........
             ........
             ......x.
             .....x..
             ....x...
             ...x....",
        )
        .unwrap();

        let position = bitboard_from_str(
            "........
             ........
             ........
             .......x
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let calculated_ray_attack = calculate_soeast_ray_attack(position);

        assert_eq!(correct_ray_attack, calculated_ray_attack)
    }

    #[test]
    fn test_calculate_north_east_ray_attack() {
        let correct_ray_attack = bitboard_from_str(
            "..x.....
             ...x....
             ....x...
             .....x..
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let position = bitboard_from_str(
            "........
             ........
             ........
             ........
             ......x.
             ........
             ........
             ........",
        )
        .unwrap();

        let calculated_ray_attack = calculate_noeast_ray_attack(position);

        assert_eq!(correct_ray_attack, calculated_ray_attack)
    }

    #[test]
    fn test_calculate_south_west_ray_attack() {
        let correct_ray_attack = bitboard_from_str(
            "........
             ........
             ........
             .x......
             ..x.....
             ...x....
             ....x...
             .....x..",
        )
        .unwrap();

        let position = bitboard_from_str(
            "........
             ........
             x.......
             ........
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let calculated_ray_attack = calculate_sowest_ray_attack(position);

        assert_eq!(correct_ray_attack, calculated_ray_attack)
    }

    // #[test]
    // fn test_calculate_ray_collection_for_each_square() {
    //     let ray_collection_all = calculate_ray_collection_for_each_square();
    //
    //     assert_eq!(ray_collection_all.len(), SQUARE_COUNT)
    // }
    //
    #[test]
    fn test_ray_collection() {
        let ray_collection = ray_collection(55);

        assert!(ray_collection.contains_key(&RayDirection::EAST))
    }
}
