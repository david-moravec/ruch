use int_enum::IntEnum;
use std::hash::Hash;
use strum_macros::EnumIter;

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

use crate::types::file::File;
use crate::types::rank::Rank;

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
