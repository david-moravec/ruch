use int_enum::IntEnum;
use std::hash::Hash;
use strum_macros::EnumIter;

use crate::constants::FULL_BOARD;
use crate::types::bitboard::BitBoard;
use crate::types::square::{Square, Square::*};

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

use crate::constants::{
    EIGHT_RANK, FIVE_RANK, FOUR_RANK, ONE_RANK, SEVEN_RANK, SIX_RANK, THREE_RANK, TWO_RANK,
};

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

    pub const fn mask(self) -> BitBoard {
        match self {
            Rank::ONE => ONE_RANK,
            Rank::TWO => TWO_RANK,
            Rank::THREE => THREE_RANK,
            Rank::FOUR => FOUR_RANK,
            Rank::FIVE => FIVE_RANK,
            Rank::SIX => SIX_RANK,
            Rank::SEVEN => SEVEN_RANK,
            Rank::EIGHT => EIGHT_RANK,
        }
    }

    pub const fn mask_till(self) -> BitBoard {
        FULL_BOARD >> (64 - self as u64)
    }

    pub const fn mask_from(self) -> BitBoard {
        FULL_BOARD << 1
    }
}
