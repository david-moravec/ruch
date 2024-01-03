use crate::bitboard::BitBoard;

pub const ONE: u64 = 1;
pub const ZERO: u64 = 0;
pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

pub const ROW_COUNT: usize = 8;
pub const FILE_COUNT: usize = 8;
pub const SQUARE_COUNT: usize = ROW_COUNT * FILE_COUNT;

pub const NOT_A_FILE: BitBoard = 0xfefefefefefefefe;
pub const NOT_H_FILE: BitBoard = 0x7f7f7f7f7f7f7f7f;
pub const NOT_AB_FILE: u64 = NOT_A_FILE & NOT_A_FILE << 1;
pub const NOT_GH_FILE: u64 = NOT_H_FILE & NOT_H_FILE >> 1;
