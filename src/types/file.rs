use int_enum::IntEnum;
use std::hash::Hash;
use strum_macros::EnumIter;

use crate::constants::{A_FILE, B_FILE, C_FILE, D_FILE, E_FILE, F_FILE, G_FILE, H_FILE};
use crate::types::bitboard::BitBoard;
use crate::types::square::Square;
use crate::types::square::Square::*;

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

    pub const fn mask(self) -> BitBoard {
        match self {
            File::A => A_FILE,
            File::B => B_FILE,
            File::C => C_FILE,
            File::D => D_FILE,
            File::E => E_FILE,
            File::F => F_FILE,
            File::G => G_FILE,
            File::H => H_FILE,
        }
    }
}
