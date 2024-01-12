use crate::{
    bitboard::{board_flat, BitBoard},
    constants::A_FILE,
};
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

fn calculate_ray_attacks_for_each_direction(origin: BitBoard) -> HashMap<RayDirection, BitBoard> {
    let mut result: HashMap<RayDirection, BitBoard> = HashMap::new();
    let lsb: u32 = origin.trailing_zeros();

    result.insert(RayDirection::NORTH, {
        let source: BitBoard = A_FILE;
        source << lsb
    });

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bitboard::{bitboard_from_str, bitboard_to_str};

    #[test]
    fn test_calculate_ray_attacks_for_each_direction() {
        let d4 = bitboard_from_str(
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

        let north_ray_d4 = bitboard_from_str(
            "...x....
             ...x....
             ...x....
             ...x....
             ........
             ........
             ........
             ........",
        )
        .unwrap();

        let to_test_rays_d4 = calculate_ray_attacks_for_each_direction(d4);

        if let Some(to_test_north_ray_d4) = to_test_rays_d4.get(&RayDirection::NORTH) {
            println!("{}", bitboard_to_str(*to_test_north_ray_d4));

            assert_eq!(to_test_north_ray_d4, &north_ray_d4)
        }
    }
}
