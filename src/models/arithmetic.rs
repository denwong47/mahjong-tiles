//! Implementation of basic arithmetic operations on [`MahjongTile`].
//!

use std::ops::{Add, Sub};

use crate::models::{IsMahjongTile, MahjongTile};

impl Add<u8> for MahjongTile {
    type Output = Option<Self>;

    fn add(self, other: u8) -> Self::Output {
        self.value()
            .and_then(|value| value.checked_add(other))
            .and_then(|new_value| {
                if new_value > 9 {
                    None
                } else {
                    Some(MahjongTile::new_valued(self.category(), new_value))
                }
            })
    }
}

impl Sub<u8> for MahjongTile {
    type Output = Option<Self>;

    fn sub(self, other: u8) -> Self::Output {
        self.value()
            .and_then(|value| value.checked_sub(other))
            .and_then(|new_value| {
                if new_value == 0 {
                    None
                } else {
                    Some(MahjongTile::new_valued(self.category(), new_value))
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

    create_test!(
        tong1_add_1,
        MahjongTile::TONG(1),
        1,
        Some(MahjongTile::TONG(2))
    );
    create_test!(
        tong1_add_2,
        MahjongTile::TONG(1),
        2,
        Some(MahjongTile::TONG(3))
    );
    create_test!(
        tong1_add_8,
        MahjongTile::TONG(1),
        8,
        Some(MahjongTile::TONG(9))
    );
    create_test!(tong9_add_1, MahjongTile::TONG(9), 1, None);
    create_test!(tong9_add_2, MahjongTile::TONG(9), 2, None);
    create_test!(
        tong9_add_0,
        MahjongTile::TONG(9),
        0,
        Some(MahjongTile::TONG(9))
    );
    create_test!(
        wan1_add_1,
        MahjongTile::WAN(1),
        1,
        Some(MahjongTile::WAN(2))
    );
    create_test!(
        wan1_add_2,
        MahjongTile::WAN(1),
        2,
        Some(MahjongTile::WAN(3))
    );
    create_test!(
        wan1_add_8,
        MahjongTile::WAN(1),
        8,
        Some(MahjongTile::WAN(9))
    );
    create_test!(wan9_add_1, MahjongTile::WAN(9), 1, None);
    create_test!(wan9_add_2, MahjongTile::WAN(9), 2, None);
    create_test!(
        wan9_add_0,
        MahjongTile::WAN(9),
        0,
        Some(MahjongTile::WAN(9))
    );
    create_test!(
        tiao1_add_1,
        MahjongTile::TIAO(1),
        1,
        Some(MahjongTile::TIAO(2))
    );
    create_test!(
        tiao1_add_2,
        MahjongTile::TIAO(1),
        2,
        Some(MahjongTile::TIAO(3))
    );
    create_test!(
        tiao1_add_8,
        MahjongTile::TIAO(1),
        8,
        Some(MahjongTile::TIAO(9))
    );
    create_test!(tiao9_add_1, MahjongTile::TIAO(9), 1, None);
    create_test!(tiao9_add_2, MahjongTile::TIAO(9), 2, None);
    create_test!(
        tiao9_add_0,
        MahjongTile::TIAO(9),
        0,
        Some(MahjongTile::TIAO(9))
    );
    create_test!(plum_add_1, MahjongTile::PLUM, 1, None);
    create_test!(east_add_1, MahjongTile::EAST, 1, None);
    create_test!(central_add_1, MahjongTile::CENTRAL, 1, None);
    create_test!(spring_add_1, MahjongTile::SPRING, 1, None);
    create_test!(tong9_add_255, MahjongTile::TONG(9), 255, None);
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

    create_test!(tong1_sub_1, MahjongTile::TONG(1), 1, None);
    create_test!(tong1_sub_2, MahjongTile::TONG(1), 2, None);
    create_test!(
        tong1_sub_0,
        MahjongTile::TONG(1),
        0,
        Some(MahjongTile::TONG(1))
    );
    create_test!(
        tong2_sub_1,
        MahjongTile::TONG(2),
        1,
        Some(MahjongTile::TONG(1))
    );
    create_test!(
        tong9_sub_1,
        MahjongTile::TONG(9),
        1,
        Some(MahjongTile::TONG(8))
    );
    create_test!(
        tong9_sub_2,
        MahjongTile::TONG(9),
        2,
        Some(MahjongTile::TONG(7))
    );
    create_test!(
        tong9_sub_8,
        MahjongTile::TONG(9),
        8,
        Some(MahjongTile::TONG(1))
    );
    create_test!(wan1_sub_1, MahjongTile::WAN(1), 1, None);
    create_test!(wan1_sub_2, MahjongTile::WAN(1), 2, None);
    create_test!(
        wan1_sub_0,
        MahjongTile::WAN(1),
        0,
        Some(MahjongTile::WAN(1))
    );
    create_test!(
        wan2_sub_1,
        MahjongTile::WAN(2),
        1,
        Some(MahjongTile::WAN(1))
    );
    create_test!(
        wan9_sub_1,
        MahjongTile::WAN(9),
        1,
        Some(MahjongTile::WAN(8))
    );
    create_test!(
        wan9_sub_2,
        MahjongTile::WAN(9),
        2,
        Some(MahjongTile::WAN(7))
    );
    create_test!(
        wan9_sub_8,
        MahjongTile::WAN(9),
        8,
        Some(MahjongTile::WAN(1))
    );
    create_test!(tiao1_sub_1, MahjongTile::TIAO(1), 1, None);
    create_test!(tiao1_sub_2, MahjongTile::TIAO(1), 2, None);
    create_test!(
        tiao1_sub_0,
        MahjongTile::TIAO(1),
        0,
        Some(MahjongTile::TIAO(1))
    );
    create_test!(
        tiao2_sub_1,
        MahjongTile::TIAO(2),
        1,
        Some(MahjongTile::TIAO(1))
    );
    create_test!(
        tiao9_sub_1,
        MahjongTile::TIAO(9),
        1,
        Some(MahjongTile::TIAO(8))
    );
    create_test!(
        tiao9_sub_2,
        MahjongTile::TIAO(9),
        2,
        Some(MahjongTile::TIAO(7))
    );
    create_test!(
        tiao9_sub_8,
        MahjongTile::TIAO(9),
        8,
        Some(MahjongTile::TIAO(1))
    );
    create_test!(plum_sub_1, MahjongTile::PLUM, 1, None);
    create_test!(east_sub_1, MahjongTile::EAST, 1, None);
    create_test!(central_sub_1, MahjongTile::CENTRAL, 1, None);
    create_test!(spring_sub_1, MahjongTile::SPRING, 1, None);
    create_test!(tong9_sub_255, MahjongTile::TONG(9), 255, None);
}
