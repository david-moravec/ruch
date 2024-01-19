use crate::types::bitboard::BitBoard;

pub const ONE: u64 = 1;
pub const ZERO: u64 = 0;
pub const DEFAULT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

pub const ROW_COUNT: usize = 8;
pub const FILE_COUNT: usize = 8;
pub const SQUARE_COUNT: usize = ROW_COUNT * FILE_COUNT;

pub const A_FILE: BitBoard = 0x0101010101010101;
pub const B_FILE: BitBoard = 0x0202020202020202;
pub const C_FILE: BitBoard = 0x0404040404040404;
pub const D_FILE: BitBoard = 0x0808080808080808;
pub const E_FILE: BitBoard = 0x1010101010101010;
pub const F_FILE: BitBoard = 0x2020202020202020;
pub const G_FILE: BitBoard = 0x4040404040404040;
pub const H_FILE: BitBoard = 0x8080808080808080;

pub const ONE_RANK: BitBoard = 0x00000000000000FF;
pub const TWO_RANK: BitBoard = 0x000000000000FF00;
pub const THREE_RANK: BitBoard = 0x0000000000FF0000;
pub const FOUR_RANK: BitBoard = 0x00000000FF000000;
pub const FIVE_RANK: BitBoard = 0x000000FF00000000;
pub const SIX_RANK: BitBoard = 0x0000FF0000000000;
pub const SEVEN_RANK: BitBoard = 0x00FF000000000000;
pub const EIGHT_RANK: BitBoard = 0xFF00000000000000;

pub const FULL_BOARD: BitBoard = 0xFFFFFFFFFFFFFFFF;

pub const A1_H8_DIAG: BitBoard = 0x8040201008040201;
pub const H1_A8_DIAG: BitBoard = 0x0102040810204080;
pub const NOT_A_FILE: BitBoard = !A_FILE;
pub const NOT_H_FILE: BitBoard = !H_FILE;

pub const NOT_AB_FILE: BitBoard = NOT_A_FILE & NOT_A_FILE << 1;
pub const NOT_ABC_FILE: BitBoard = NOT_A_FILE & NOT_AB_FILE << 1;
pub const NOT_ABCD_FILE: BitBoard = NOT_A_FILE & NOT_ABC_FILE << 1;
pub const NOT_ABCDE_FILE: BitBoard = NOT_A_FILE & NOT_ABCD_FILE << 1;
pub const NOT_ABCDEF_FILE: BitBoard = NOT_A_FILE & NOT_ABCDE_FILE << 1;
pub const NOT_ABCDEFG_FILE: BitBoard = NOT_A_FILE & NOT_ABCDEF_FILE << 1;

pub const NOT_GH_FILE: BitBoard = NOT_H_FILE & NOT_H_FILE >> 1;
pub const NOT_FGH_FILE: BitBoard = NOT_H_FILE & NOT_GH_FILE >> 1;
pub const NOT_EFGH_FILE: BitBoard = NOT_H_FILE & NOT_FGH_FILE >> 1;
pub const NOT_DEFGH_FILE: BitBoard = NOT_H_FILE & NOT_EFGH_FILE >> 1;
pub const NOT_CDEFGH_FILE: BitBoard = NOT_H_FILE & NOT_DEFGH_FILE >> 1;
pub const NOT_BCDEFGH_FILE: BitBoard = NOT_H_FILE & NOT_CDEFGH_FILE >> 1;
