use int_enum::IntEnum;
use strum_macros::EnumIter;

#[repr(u64)]
#[derive(EnumIter, Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum Square {
    A1 = 0, B1 = 8,  C1 = 16, D1 = 24, E1 = 32,F1 = 40, G1 = 48, H1 = 56,
    A2 = 1, B2 = 9,  C2 = 17, D2 = 25, E2 = 33,F2 = 41, G2 = 49, H2 = 57,
    A3 = 2, B3 = 10, C3 = 18, D3 = 26, E3 = 34,F3 = 42, G3 = 50, H3 = 58,
    A4 = 3, B4 = 11, C4 = 19, D4 = 27, E4 = 35,F4 = 43, G4 = 51, H4 = 59,
    A5 = 4, B5 = 12, C5 = 20, D5 = 28, E5 = 36,F5 = 44, G5 = 52, H5 = 60,
    A6 = 5, B6 = 13, C6 = 21, D6 = 29, E6 = 37,F6 = 45, G6 = 53, H6 = 61,
    A7 = 6, B7 = 14, C7 = 22, D7 = 30, E7 = 38,F7 = 46, G7 = 54, H7 = 62,
    A8 = 7, B8 = 15, C8 = 23, D8 = 31, E8 = 39,F8 = 47, G8 = 55, H8 = 63,
}

use Square::*;

impl TryFrom<u64> for Square {
    type Error = &'static str;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(A1), 8 => Ok(B1),  16 => Ok(C1), 24 => Ok(D1), 32 => Ok(E1), 40 => Ok(F1), 48 => Ok(G1), 56 => Ok(H1),
            1 => Ok(A2), 9 => Ok(B2),  17 => Ok(C2), 25 => Ok(D2), 33 => Ok(E2), 41 => Ok(F2), 49 => Ok(G2), 57 => Ok(H2),
            2 => Ok(A3), 10 => Ok(B3), 18 => Ok(C3), 26 => Ok(D3), 34 => Ok(E3), 42 => Ok(F3), 50 => Ok(G3), 58 => Ok(H3),
            3 => Ok(A4), 11 => Ok(B4), 19 => Ok(C4), 27 => Ok(D4), 35 => Ok(E4), 43 => Ok(F4), 51 => Ok(G4), 59 => Ok(H4),
            4 => Ok(A5), 12 => Ok(B5), 20 => Ok(C5), 28 => Ok(D5), 36 => Ok(E5), 44 => Ok(F5), 52 => Ok(G5), 60 => Ok(H5),
            5 => Ok(A6), 13 => Ok(B6), 21 => Ok(C6), 29 => Ok(D6), 37 => Ok(E6), 45 => Ok(F6), 53 => Ok(G6), 61 => Ok(H6),
            6 => Ok(A7), 14 => Ok(B7), 22 => Ok(C7), 30 => Ok(D7), 38 => Ok(E7), 46 => Ok(F7), 54 => Ok(G7), 62 => Ok(H7),
            7 => Ok(A8), 15 => Ok(B8), 23 => Ok(C8), 31 => Ok(D8), 39 => Ok(E8), 47 => Ok(F8), 55 => Ok(G8), 63 => Ok(H8),
            _ => Err("Cannot convert given int to Square"),
        }
    }
}

