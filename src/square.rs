use int_enum::IntEnum;
use std::hash::Hash;
use strum_macros::EnumIter;

pub const ROW_COUNT: usize = 8;
pub const FILE_COUNT: usize = 8;
pub const SQUARE_COUNT: usize = ROW_COUNT * FILE_COUNT;

#[repr(u64)]
#[derive(EnumIter, Clone, Copy, Debug, Eq, PartialEq, IntEnum, Hash)]
pub enum Square {
    A1 = 0,
    B1 = 1,
    C1 = 2,
    D1 = 3,
    E1 = 4,
    F1 = 5,
    G1 = 6,
    H1 = 7,
    A2 = 8,
    B2 = 9,
    C2 = 10,
    D2 = 11,
    E2 = 12,
    F2 = 13,
    G2 = 14,
    H2 = 15,
    A3 = 16,
    B3 = 17,
    C3 = 18,
    D3 = 19,
    E3 = 20,
    F3 = 21,
    G3 = 22,
    H3 = 23,
    A4 = 24,
    B4 = 25,
    C4 = 26,
    D4 = 27,
    E4 = 28,
    F4 = 29,
    G4 = 30,
    H4 = 31,
    A5 = 32,
    B5 = 33,
    C5 = 34,
    D5 = 35,
    E5 = 36,
    F5 = 37,
    G5 = 38,
    H5 = 39,
    A6 = 40,
    B6 = 41,
    C6 = 42,
    D6 = 43,
    E6 = 44,
    F6 = 45,
    G6 = 46,
    H6 = 47,
    A7 = 48,
    B7 = 49,
    C7 = 50,
    D7 = 51,
    E7 = 52,
    F7 = 53,
    G7 = 54,
    H7 = 55,
    A8 = 56,
    B8 = 57,
    C8 = 58,
    D8 = 59,
    E8 = 60,
    F8 = 61,
    G8 = 62,
    H8 = 63,
}

use Square::*;

impl TryFrom<u64> for Square {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(A1),
            1 => Ok(B1),
            2 => Ok(C1),
            3 => Ok(D1),
            4 => Ok(E1),
            5 => Ok(F1),
            6 => Ok(G1),
            7 => Ok(H1),
            8 => Ok(A2),
            9 => Ok(B2),
            10 => Ok(C2),
            11 => Ok(D2),
            12 => Ok(E2),
            13 => Ok(F2),
            14 => Ok(G2),
            15 => Ok(H2),
            16 => Ok(A3),
            17 => Ok(B3),
            18 => Ok(C3),
            19 => Ok(D3),
            20 => Ok(E3),
            21 => Ok(F3),
            22 => Ok(G3),
            23 => Ok(H3),
            24 => Ok(A4),
            25 => Ok(B4),
            26 => Ok(C4),
            27 => Ok(D4),
            28 => Ok(E4),
            29 => Ok(F4),
            30 => Ok(G4),
            31 => Ok(H4),
            32 => Ok(A5),
            33 => Ok(B5),
            34 => Ok(C5),
            35 => Ok(D5),
            36 => Ok(E5),
            37 => Ok(F5),
            38 => Ok(G5),
            39 => Ok(H5),
            40 => Ok(A6),
            41 => Ok(B6),
            42 => Ok(C6),
            43 => Ok(D6),
            44 => Ok(E6),
            45 => Ok(F6),
            46 => Ok(G6),
            47 => Ok(H6),
            48 => Ok(A7),
            49 => Ok(B7),
            50 => Ok(C7),
            51 => Ok(D7),
            52 => Ok(E7),
            53 => Ok(F7),
            54 => Ok(G7),
            55 => Ok(H7),
            56 => Ok(A8),
            57 => Ok(B8),
            58 => Ok(C8),
            59 => Ok(D8),
            60 => Ok(E8),
            61 => Ok(F8),
            62 => Ok(G8),
            63 => Ok(H8),
            _ => Err("Cannot convert given int to Square"),
        }
    }
}

#[repr(u64)]
#[derive(EnumIter, Clone, Copy, Debug, Eq, PartialEq, IntEnum, Hash)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl TryFrom<u64> for File {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            7 => Ok(File::H),
            _ => Err("Cannot convert given int to File"),
        }
    }
}

impl File {
    pub fn squares(self) -> [Square; 8] {
        match self {
            File::A => [A1, A2, A3, A4, A5, A6, A7, A8],
            File::B => [B1, B2, B3, B4, B5, B6, B7, B8],
            File::C => [C1, C2, C3, C4, C5, C6, C7, C8],
            File::D => [D1, D2, D3, D4, D5, D6, D7, D8],
            File::E => [E1, E2, E3, E4, E5, E6, E7, E8],
            File::F => [F1, F2, F3, F4, F5, F6, F7, F8],
            File::G => [G1, G2, G3, G4, G5, G6, G7, G8],
            File::H => [H1, H2, H3, H4, H5, H6, H7, H8],
        }
    }

    pub fn to_char(self) -> char {
        match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
}

#[repr(u64)]
#[derive(EnumIter, Clone, Copy, Debug, Eq, PartialEq, IntEnum, Hash)]
pub enum Rank {
    ONE = 0,
    TWO = 1,
    THREE = 2,
    FOUR = 3,
    FIVE = 4,
    SIX = 5,
    SEVEN = 6,
    EIGHT = 7,
}

impl TryFrom<u64> for Rank {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Rank::ONE),
            1 => Ok(Rank::TWO),
            2 => Ok(Rank::THREE),
            3 => Ok(Rank::FOUR),
            4 => Ok(Rank::FIVE),
            5 => Ok(Rank::SIX),
            6 => Ok(Rank::SEVEN),
            7 => Ok(Rank::EIGHT),
            _ => Err("Cannot convert given int to File"),
        }
    }
}

impl Rank {
    pub fn squares(self) -> [Square; 8] {
        match self {
            Rank::ONE => [A1, B1, C1, D1, E1, F1, G1, H1],
            Rank::TWO => [A2, B2, C2, D2, E2, F2, G2, H2],
            Rank::THREE => [A3, B3, C3, D3, E3, F3, G3, H3],
            Rank::FOUR => [A4, B4, C4, D4, E4, F4, G4, H4],
            Rank::FIVE => [A5, B5, C5, D5, E5, F5, G5, H5],
            Rank::SIX => [A6, B6, C6, D6, E6, F6, G6, H6],
            Rank::SEVEN => [A7, B7, C7, D7, E7, F7, G7, H7],
            Rank::EIGHT => [A8, B8, C8, D8, E8, F8, G8, H8],
        }
    }
}

impl Square {
    pub fn file(self) -> Option<File> {
        File::try_from(self as u64 & 7).ok()
    }

    pub fn rank(self) -> Option<Rank> {
        Rank::try_from(self as u64 >> 3).ok()
    }

    pub const fn as_bitboard(self) -> u64 {
        1 << (self as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::{File, Rank, Square};

    #[test]
    fn test_square_2_rank() {
        assert_eq!(Square::B4.rank(), Some(Rank::FOUR));
    }

    #[test]
    fn test_square_2_file() {
        assert_eq!(Square::C4.file(), Some(File::C));
    }
}
