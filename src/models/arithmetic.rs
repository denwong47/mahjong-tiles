//! Implementation of basic arithmetic operations on [`Tile`].
//!

use std::ops::{Add, Sub};

use crate::models::{IsTile, Tile};

impl Add<u8> for Tile {
    type Output = Option<Self>;

    fn add(self, other: u8) -> Self::Output {
        self.value()
            .and_then(|value| value.checked_add(other))
            .and_then(|new_value| {
                if new_value > 9 {
                    None
                } else {
                    Some(Tile::new_valued(self.category(), new_value))
                }
            })
    }
}

impl Sub<u8> for Tile {
    type Output = Option<Self>;

    fn sub(self, other: u8) -> Self::Output {
        self.value()
            .and_then(|value| value.checked_sub(other))
            .and_then(|new_value| {
                if new_value == 0 {
                    None
                } else {
                    Some(Tile::new_valued(self.category(), new_value))
                }
            })
    }
}

#[cfg(test)]
mod test_add {
    use super::*;

    macro_rules! create_test {
        ($name:ident, $tile:expr, $other:literal, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!($tile + $other, $expected);
            }
        };
    }

    create_test!(tong1_add_1, Tile::TONG(1), 1, Some(Tile::TONG(2)));
    create_test!(tong1_add_2, Tile::TONG(1), 2, Some(Tile::TONG(3)));
    create_test!(tong1_add_8, Tile::TONG(1), 8, Some(Tile::TONG(9)));
    create_test!(tong9_add_1, Tile::TONG(9), 1, None);
    create_test!(tong9_add_2, Tile::TONG(9), 2, None);
    create_test!(tong9_add_0, Tile::TONG(9), 0, Some(Tile::TONG(9)));
    create_test!(wan1_add_1, Tile::WAN(1), 1, Some(Tile::WAN(2)));
    create_test!(wan1_add_2, Tile::WAN(1), 2, Some(Tile::WAN(3)));
    create_test!(wan1_add_8, Tile::WAN(1), 8, Some(Tile::WAN(9)));
    create_test!(wan9_add_1, Tile::WAN(9), 1, None);
    create_test!(wan9_add_2, Tile::WAN(9), 2, None);
    create_test!(wan9_add_0, Tile::WAN(9), 0, Some(Tile::WAN(9)));
    create_test!(tiao1_add_1, Tile::TIAO(1), 1, Some(Tile::TIAO(2)));
    create_test!(tiao1_add_2, Tile::TIAO(1), 2, Some(Tile::TIAO(3)));
    create_test!(tiao1_add_8, Tile::TIAO(1), 8, Some(Tile::TIAO(9)));
    create_test!(tiao9_add_1, Tile::TIAO(9), 1, None);
    create_test!(tiao9_add_2, Tile::TIAO(9), 2, None);
    create_test!(tiao9_add_0, Tile::TIAO(9), 0, Some(Tile::TIAO(9)));
    create_test!(plum_add_1, Tile::PLUM, 1, None);
    create_test!(east_add_1, Tile::EAST, 1, None);
    create_test!(central_add_1, Tile::CENTRAL, 1, None);
    create_test!(spring_add_1, Tile::SPRING, 1, None);
    create_test!(tong9_add_255, Tile::TONG(9), 255, None);
}

#[cfg(test)]
mod test_sub {
    use super::*;

    macro_rules! create_test {
        ($name:ident, $tile:expr, $other:literal, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!($tile - $other, $expected);
            }
        };
    }

    create_test!(tong1_sub_1, Tile::TONG(1), 1, None);
    create_test!(tong1_sub_2, Tile::TONG(1), 2, None);
    create_test!(tong1_sub_0, Tile::TONG(1), 0, Some(Tile::TONG(1)));
    create_test!(tong2_sub_1, Tile::TONG(2), 1, Some(Tile::TONG(1)));
    create_test!(tong9_sub_1, Tile::TONG(9), 1, Some(Tile::TONG(8)));
    create_test!(tong9_sub_2, Tile::TONG(9), 2, Some(Tile::TONG(7)));
    create_test!(tong9_sub_8, Tile::TONG(9), 8, Some(Tile::TONG(1)));
    create_test!(wan1_sub_1, Tile::WAN(1), 1, None);
    create_test!(wan1_sub_2, Tile::WAN(1), 2, None);
    create_test!(wan1_sub_0, Tile::WAN(1), 0, Some(Tile::WAN(1)));
    create_test!(wan2_sub_1, Tile::WAN(2), 1, Some(Tile::WAN(1)));
    create_test!(wan9_sub_1, Tile::WAN(9), 1, Some(Tile::WAN(8)));
    create_test!(wan9_sub_2, Tile::WAN(9), 2, Some(Tile::WAN(7)));
    create_test!(wan9_sub_8, Tile::WAN(9), 8, Some(Tile::WAN(1)));
    create_test!(tiao1_sub_1, Tile::TIAO(1), 1, None);
    create_test!(tiao1_sub_2, Tile::TIAO(1), 2, None);
    create_test!(tiao1_sub_0, Tile::TIAO(1), 0, Some(Tile::TIAO(1)));
    create_test!(tiao2_sub_1, Tile::TIAO(2), 1, Some(Tile::TIAO(1)));
    create_test!(tiao9_sub_1, Tile::TIAO(9), 1, Some(Tile::TIAO(8)));
    create_test!(tiao9_sub_2, Tile::TIAO(9), 2, Some(Tile::TIAO(7)));
    create_test!(tiao9_sub_8, Tile::TIAO(9), 8, Some(Tile::TIAO(1)));
    create_test!(plum_sub_1, Tile::PLUM, 1, None);
    create_test!(east_sub_1, Tile::EAST, 1, None);
    create_test!(central_sub_1, Tile::CENTRAL, 1, None);
    create_test!(spring_sub_1, Tile::SPRING, 1, None);
    create_test!(tong9_sub_255, Tile::TONG(9), 255, None);
}
