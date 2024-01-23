use crate::board::board_flat;
use crate::constants::{NOT_A_FILE, NOT_H_FILE, ONE, ROW_COUNT, ZERO};
use crate::types::square::Square;

use strum::IntoEnumIterator;

pub type BitBoard = u64;

pub fn square_occupied(bboard: BitBoard, square: Square) -> bool {
    bboard & (ONE << square as u64) != 0
}

pub const fn north_one(bitboard: BitBoard) -> BitBoard {
    bitboard << 8
}

pub const fn west_one(bitboard: BitBoard) -> BitBoard {
    bitboard << 1 & NOT_A_FILE
}

pub const fn north_west_one(bitboard: BitBoard) -> BitBoard {
    bitboard << 9 & NOT_A_FILE
}

pub const fn north_east_one(bitboard: BitBoard) -> BitBoard {
    bitboard << 7 & NOT_H_FILE
}

pub const fn south_one(bitboard: BitBoard) -> BitBoard {
    bitboard >> 8
}

pub const fn east_one(bitboard: BitBoard) -> BitBoard {
    bitboard >> 1 & NOT_H_FILE
}

pub const fn south_east_one(bitboard: BitBoard) -> BitBoard {
    bitboard >> 9 & NOT_H_FILE
}

pub const fn south_west_one(bitboard: BitBoard) -> BitBoard {
    bitboard >> 7 & NOT_A_FILE
}

pub fn bitboard_from_str(s: &'static str) -> Result<u64, &'static str> {
    // Used for quickly generating bitboard with a occupancy specified by 'X' or '.'.
    // Inspired by cozy_chess bitboard! macro
    //
    let a = flatten_multiline_string_to_bitboard_repr(s.to_string())?;
    Ok(a.iter().enumerate().fold(
        ZERO,
        |acc, (i, c)| if *c == 'x' { acc | ONE << i } else { acc },
    ))
}

fn flatten_multiline_string_to_bitboard_repr(s: String) -> Result<Vec<char>, &'static str> {
    let mut s = s.to_string();
    s.retain(|c| !c.is_whitespace());
    let s_vec: Vec<char> = s.chars().collect();

    if s_vec.len() != 64 {
        Err("String needs to have one character for each square")
    } else {
        Ok(s_vec
            .chunks(ROW_COUNT)
            .rev()
            .flat_map(|chunk| chunk.iter())
            .map(|c| *c)
            .collect())
    }
}

pub fn bitboard_to_str(bitboard: BitBoard) -> String {
    let mut result_unreversed = board_flat('.');

    for square in Square::iter() {
        if square_occupied(bitboard, square) {
            result_unreversed[square as usize] = 'x';
        }
    }

    result_unreversed
        .chunks(ROW_COUNT)
        .rev()
        .flat_map(|chunk| chunk.iter().chain(['\n'].iter()))
        .map(|c| *c)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::constants::*;

    #[test]
    fn test_flatten_multiline_string_to_bitboard_repr() {
        let correct: Vec<char> = Vec::from_iter(
            [
                'x', '.', '.', '.', 'x', '.', '.', '.', 'x', '.', '.', '.', '.', 'x', '.', '.',
                '.', '.', '.', '.', '.', '.', 'x', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
            ]
            .iter()
            .map(|c| *c),
        );

        assert_eq!(
            correct,
            flatten_multiline_string_to_bitboard_repr(
                "........
                 ........
                 ........
                 ........
                 ........
                 ......x.
                 x....x..
                 x...x..."
                    .to_string()
            )
            .unwrap()
        )
    }

    #[test]
    fn test_bitboard_from_str() {
        let correct = 4202769;

        assert_eq!(
            correct,
            bitboard_from_str(
                "........
                 ........
                 ........
                 ........
                 ........
                 ......x.
                 x....x..
                 x...x..."
            )
            .unwrap()
        );

        let correct = 44272527353856;
        let string = "........
                      ........
                      ...x.x..
                      ..x...x.
                      ........
                      ..x...x.
                      ...x.x..
                      ........";

        assert_eq!(correct, bitboard_from_str(string).unwrap());

        let correct = 22136263676928;
        let string = "........
                      ........
                      ..x.x...
                      .x...x..
                      ........
                      .x...x..
                      ..x.x...
                      ........";

        assert_eq!(correct, bitboard_from_str(string).unwrap())
    }

    #[test]
    fn test_bitboard_to_str() {
        let n: u64 = 4202769;
        let string = "........
                      ........
                      ........
                      ........
                      ........
                      ......x.
                      x....x..
                      x...x...";

        let mut to_test = bitboard_to_str(n);
        to_test.retain(|c| !c.is_whitespace());

        let mut correct = string.to_string();
        correct.retain(|c| !c.is_whitespace());

        assert_eq!(correct, to_test);

        let n: u64 = 44272527353856;
        let string = "........
                      ........
                      ...x.x..
                      ..x...x.
                      ........
                      ..x...x.
                      ...x.x..
                      ........";

        let mut to_test = bitboard_to_str(n);
        to_test.retain(|c| !c.is_whitespace());

        let mut correct = string.to_string();
        correct.retain(|c| !c.is_whitespace());

        assert_eq!(correct, to_test)
    }

    #[test]
    fn test_south_one() {
        assert_eq!(south_one(TWO_RANK), ONE_RANK);
        assert_eq!(south_one(SIX_RANK), FIVE_RANK);
    }
    #[test]
    fn test_north_one() {
        assert_eq!(north_one(TWO_RANK), THREE_RANK);
        assert_eq!(north_one(SIX_RANK), SEVEN_RANK);
    }

    #[test]
    fn test_east_one() {
        let e5 = bitboard_from_str(
            "........
             ........
             ........
             ....x...
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let e5_east_one = bitboard_from_str(
            "........
             ........
             ........
             ...x....
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        println!("{}", bitboard_to_str(east_one(e5)));

        assert_eq!(e5_east_one, east_one(e5));
    }

    #[test]
    fn test_north_east_one() {
        let e5 = bitboard_from_str(
            "........
             ........
             ........
             ....x...
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let correct = bitboard_from_str(
            "........
             ........
             ...x....
             ........
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let to_test = north_east_one(e5);

        println!("{}", bitboard_to_str(to_test));

        assert_eq!(correct, to_test);
    }

    #[test]
    fn test_north_west_one() {
        let e5 = bitboard_from_str(
            "........
             ........
             ........
             ....x...
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let correct = bitboard_from_str(
            "........
             ........
             .....x..
             ........
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let to_test = north_west_one(e5);

        println!("{}", bitboard_to_str(to_test));

        assert_eq!(correct, to_test);
    }

    #[test]
    fn test_south_west_one() {
        let e5 = bitboard_from_str(
            "........
             ........
             ........
             ....x...
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let correct = bitboard_from_str(
            "........
             ........
             ........
             ........
             .....x..
             ........
             ........
             ........",
        )
        .unwrap();

        let to_test = south_west_one(e5);

        println!("{}", bitboard_to_str(to_test));

        assert_eq!(correct, to_test);
    }

    #[test]
    fn test_south_east_one() {
        let e5 = bitboard_from_str(
            "........
             ........
             ........
             ....x...
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let correct = bitboard_from_str(
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

        let to_test = south_east_one(e5);

        println!("{}", bitboard_to_str(to_test));

        assert_eq!(correct, to_test);
    }
}
