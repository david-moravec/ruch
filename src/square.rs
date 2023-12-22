use int_enum::IntEnum;
use strum_macros::EnumIter;
use std::hash::Hash;

pub const ROW_COUNT: usize = 8;
pub const FILE_COUNT: usize = 8;
pub const SQUARE_COUNT: usize = ROW_COUNT * FILE_COUNT;

#[repr(u64)]
#[derive(EnumIter, Clone, Copy, Debug, Eq, PartialEq, IntEnum, Hash)]
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
        match self{ 
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
        match self{
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
    ONE   = 0,
    TWO   = 1,
    THREE = 2,
    FOUR  = 3,
    FIVE  = 4,
    SIX   = 5,
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
           Rank::ONE   => [A1, B1, C1, D1, E1, F1, G1, H1],
           Rank::TWO   => [A2, B2, C2, D2, E2, F2, G2, H2],
           Rank::THREE => [A3, B3, C3, D3, E3, F3, G3, H3],
           Rank::FOUR  => [A4, B4, C4, D4, E4, F4, G4, H4],
           Rank::FIVE  => [A5, B5, C5, D5, E5, F5, G5, H5],
           Rank::SIX   => [A6, B6, C6, D6, E6, F6, G6, H6],
           Rank::SEVEN => [A7, B7, C7, D7, E7, F7, G7, H7],
           Rank::EIGHT => [A8, B8, C8, D8, E8, F8, G8, H8],
        }
    }
}

impl Square {
    pub fn rank(self) -> Option<Rank> {
        Rank::try_from(self as u64 & 7).ok() 
    }

    pub fn file(self) -> Option<File> {
        File::try_from(self as u64 >> 3).ok()
    }
     
    pub const fn as_bitboard(self) -> u64 {
        1 << (self as u64)
    }
}


#[cfg(test)]
mod tests {
    use super::{Square, Rank, File};

    #[test]
    fn test_square_2_rank() {
        assert_eq!(Square::B4.rank(), Some(Rank::FOUR));
    }

    #[test]
    fn test_square_2_file() {
        assert_eq!(Square::C4.file(), Some(File::C));
    }
} 
