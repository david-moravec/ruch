use crate::bitboard::{board_flat, BitBoard};
use std::collections::HashMap;

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

fn generate_ray_attack(origin: BitBoard) -> HashMap<RayDirection, BitBoard> {
    let mut result = board_flat(0 as u64);

    HashMap::from([(RayDirection::NORTH, 1 as BitBoard)])
}
