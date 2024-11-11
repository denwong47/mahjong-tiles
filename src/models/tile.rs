use super::{FlowerCategory, HasChineseValue, HonourCategory, TileCategory};

/// A single Mahjong tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MahjongTile {
    /// The East wind.
    EAST,
    /// The South wind.
    SOUTH,
    /// The West wind.
    WEST,
    /// The North wind.
    NORTH,
    /// The Central dragon, also known as the Red.
    CENTRAL,
    /// The Prosperity dragon, also known as the Green.
    PROSPERITY,
    /// The Blank dragon, also known as the White.
    BLANK,
    /// The Tong tiles, numbered 1 to 9.
    TONG(u8),
    /// The Wan tiles, numbered 1 to 9.
    WAN(u8),
    /// The Tiao tiles, numbered 1 to 9.
    TIAO(u8),

    // Flower tiles - Not commonly used, no SVG data available.
    /// The Plum flower for Winter.
    PLUM,
    /// The Orchid flower for Spring.
    ORCHID,
    /// The Chrysanthemum flower for Autumn.
    CHRYSANTHEMUM,
    /// The Bamboo flower for Summer.
    BAMBOO,

    // Season tiles - Not commonly used, no SVG data available.
    /// The Spring season.
    SPRING,
    /// The Summer season.
    SUMMER,
    /// The Autumn season.
    AUTUMN,
    /// The Winter season.
    WINTER,
}

impl PartialOrd for MahjongTile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.category() == other.category() && !self.is_honour() {
            if let (Some(v1), Some(v2)) = (self.value(), other.value()) {
                v1.partial_cmp(&v2)
            } else {
                unreachable!("Unreachable: Number tiles should have a value")
            }
        } else {
            None
        }
    }
}

impl MahjongTile {
    /// Returns the chinese name for the tile.
    pub fn chinese_name(&self) -> String {
        match self {
            MahjongTile::EAST => "东".to_string(),
            MahjongTile::SOUTH => "南".to_string(),
            MahjongTile::WEST => "西".to_string(),
            MahjongTile::NORTH => "北".to_string(),
            MahjongTile::CENTRAL => "中".to_string(),
            MahjongTile::PROSPERITY => "发".to_string(),
            MahjongTile::BLANK => "白".to_string(),
            MahjongTile::TIAO(v) | MahjongTile::TONG(v) | MahjongTile::WAN(v) => {
                let category_name = self.category().category_name_chinese();
                let chinese_value = v.chinese_value();
                format!("{}{}", chinese_value, category_name)
            }
            MahjongTile::PLUM => "梅".to_string(),
            MahjongTile::ORCHID => "兰".to_string(),
            MahjongTile::CHRYSANTHEMUM => "菊".to_string(),
            MahjongTile::BAMBOO => "竹".to_string(),
            MahjongTile::SPRING => "春".to_string(),
            MahjongTile::SUMMER => "夏".to_string(),
            MahjongTile::AUTUMN => "秋".to_string(),
            MahjongTile::WINTER => "冬".to_string(),
        }
    }
}

/// A trait for Mahjong tiles.
pub trait IsMahjongTile: PartialOrd + Eq + std::fmt::Debug {
    /// Returns the category of the tile.
    fn category(&self) -> TileCategory;
    /// Returns the value of the tile, if it has one.
    fn value(&self) -> Option<u8>;

    /// Returns whether the tile is an honour tile.
    fn is_honour(&self) -> bool {
        matches!(self.category(), TileCategory::HONOUR(_))
    }

    /// Returns whether the tile is a number tile.
    fn is_number(&self) -> bool {
        matches!(
            self.category(),
            TileCategory::TONG | TileCategory::WAN | TileCategory::TIAO
        )
    }

    /// Returns whether the tile is a flower tile.
    fn is_flower(&self) -> bool {
        matches!(self.category(), TileCategory::FLOWER(_))
    }

    /// Get the SVG name for this tile.
    fn svg_name(&self) -> String {
        let category_name = self.category().category_name().to_ascii_uppercase();

        if self.is_honour() {
            format!("{}_{:?}", category_name, self)
        } else {
            format!(
                "{}_{}",
                category_name,
                self.value()
                    .expect("Unreachable: Number tiles should have a value")
            )
        }
    }
}

impl IsMahjongTile for MahjongTile {
    fn category(&self) -> TileCategory {
        match self {
            MahjongTile::EAST | MahjongTile::SOUTH | MahjongTile::WEST | MahjongTile::NORTH => {
                TileCategory::HONOUR(HonourCategory::WIND)
            }
            MahjongTile::CENTRAL | MahjongTile::PROSPERITY | MahjongTile::BLANK => {
                TileCategory::HONOUR(HonourCategory::DRAGON)
            }
            MahjongTile::TONG(_) => TileCategory::TONG,
            MahjongTile::WAN(_) => TileCategory::WAN,
            MahjongTile::TIAO(_) => TileCategory::TIAO,
            MahjongTile::PLUM
            | MahjongTile::ORCHID
            | MahjongTile::CHRYSANTHEMUM
            | MahjongTile::BAMBOO => TileCategory::FLOWER(FlowerCategory::FLOWER),
            MahjongTile::SPRING
            | MahjongTile::SUMMER
            | MahjongTile::AUTUMN
            | MahjongTile::WINTER => TileCategory::FLOWER(FlowerCategory::SEASON),
        }
    }

    fn value(&self) -> Option<u8> {
        match self {
            MahjongTile::TONG(v) | MahjongTile::WAN(v) | MahjongTile::TIAO(v) => Some(*v),
            _ => None,
        }
    }
}

// =====================================================================================
// Unit tests
// =====================================================================================

#[cfg(test)]
mod test_eq {
    use super::*;

    macro_rules! create_test {
        ($name:ident, $lhs:expr, $rhs:expr, $expected:literal) => {
            #[test]
            fn $name() {
                if $expected {
                    assert_eq!($lhs, $rhs);
                } else {
                    assert_ne!($lhs, $rhs);
                }
            }
        };
    }

    create_test!(east_east, MahjongTile::EAST, MahjongTile::EAST, true);
    create_test!(east_south, MahjongTile::EAST, MahjongTile::SOUTH, false);
    create_test!(east_tong1, MahjongTile::EAST, MahjongTile::TONG(1), false);
    create_test!(
        tong1_tong1,
        MahjongTile::TONG(1),
        MahjongTile::TONG(1),
        true
    );
    create_test!(
        tong1_tong2,
        MahjongTile::TONG(1),
        MahjongTile::TONG(2),
        false
    );
    create_test!(tong1_wan1, MahjongTile::TONG(1), MahjongTile::WAN(1), false);
    create_test!(
        tong1_central,
        MahjongTile::TONG(1),
        MahjongTile::CENTRAL,
        false
    );
    create_test!(
        tong1_tiao1,
        MahjongTile::TONG(1),
        MahjongTile::TIAO(1),
        false
    );
}

#[cfg(test)]
#[allow(clippy::neg_cmp_op_on_partial_ord)]
mod test_ord {
    use super::*;
    use std::cmp::Ordering;
    macro_rules! create_test {
        ($name:ident, $st:expr, $expected:literal) => {
            #[test]
            fn $name() {
                if $expected {
                    assert!($st);
                } else {
                    assert!(!($st));
                }
            }
        };
        ($name:ident, $lhs:expr, $rhs:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!($lhs.partial_cmp(&$rhs), $expected);
            }
        };
    }

    create_test!(east_cmp_east, MahjongTile::EAST, MahjongTile::EAST, None);
    create_test!(east_cmp_south, MahjongTile::EAST, MahjongTile::SOUTH, None);
    create_test!(east_lt_east, MahjongTile::EAST < MahjongTile::EAST, false);

    create_test!(east_le_east, MahjongTile::EAST <= MahjongTile::EAST, false);
    create_test!(east_gt_east, MahjongTile::EAST > MahjongTile::EAST, false);
    create_test!(east_ge_east, MahjongTile::EAST >= MahjongTile::EAST, false);
    create_test!(east_lt_south, MahjongTile::EAST < MahjongTile::SOUTH, false);
    create_test!(
        east_lt_central,
        MahjongTile::EAST < MahjongTile::CENTRAL,
        false
    );
    create_test!(
        east_lt_tong1,
        MahjongTile::EAST < MahjongTile::TONG(1),
        false
    );
    create_test!(east_lt_wan1, MahjongTile::EAST < MahjongTile::WAN(1), false);
    create_test!(
        east_lt_tiao1,
        MahjongTile::EAST < MahjongTile::TIAO(1),
        false
    );
    create_test!(
        central_cmp_central,
        MahjongTile::CENTRAL,
        MahjongTile::CENTRAL,
        None
    );
    create_test!(
        central_lt_central,
        MahjongTile::CENTRAL < MahjongTile::CENTRAL,
        false
    );
    create_test!(
        central_cmp_prosperity,
        MahjongTile::CENTRAL,
        MahjongTile::PROSPERITY,
        None
    );
    create_test!(
        central_lt_prosperity,
        MahjongTile::CENTRAL < MahjongTile::PROSPERITY,
        false
    );
    create_test!(
        tong1_cmp_tong1,
        MahjongTile::TONG(1),
        MahjongTile::TONG(1),
        Some(Ordering::Equal)
    );
    create_test!(
        tong1_lt_tong1,
        MahjongTile::TONG(1) < MahjongTile::TONG(1),
        false
    );
    create_test!(
        tong1_le_tong1,
        MahjongTile::TONG(1) <= MahjongTile::TONG(1),
        true
    );
    create_test!(
        tong1_cmp_tong2,
        MahjongTile::TONG(1),
        MahjongTile::TONG(2),
        Some(Ordering::Less)
    );
    create_test!(
        tong1_lt_tong2,
        MahjongTile::TONG(1) < MahjongTile::TONG(2),
        true
    );
    create_test!(
        tong1_cmp_wan1,
        MahjongTile::TONG(1),
        MahjongTile::WAN(1),
        None
    );
    create_test!(
        tong1_lt_wan1,
        MahjongTile::TONG(1) < MahjongTile::WAN(1),
        false
    );
    create_test!(
        tong1_cmp_central,
        MahjongTile::TONG(1),
        MahjongTile::CENTRAL,
        None
    );
    create_test!(
        tong1_lt_central,
        MahjongTile::TONG(1) < MahjongTile::CENTRAL,
        false
    );
    create_test!(
        tong1_cmp_tiao1,
        MahjongTile::TONG(1),
        MahjongTile::TIAO(1),
        None
    );
    create_test!(
        tong1_lt_tiao1,
        MahjongTile::TONG(1) < MahjongTile::TIAO(1),
        false
    );
    create_test!(
        tong9_cmp_tong1,
        MahjongTile::TONG(9),
        MahjongTile::TONG(1),
        Some(Ordering::Greater)
    );
    create_test!(
        tong9_lt_tong1,
        MahjongTile::TONG(9) < MahjongTile::TONG(1),
        false
    );
    create_test!(
        tong9_ge_tong1,
        MahjongTile::TONG(9) >= MahjongTile::TONG(1),
        true
    );
    create_test!(
        tong9_ge_wan1,
        MahjongTile::TONG(9) >= MahjongTile::WAN(1),
        false
    );
}

#[cfg(test)]
mod test_metadata {
    use super::*;

    macro_rules! create_test {
        ($name:ident, $tile:expr, $attr:ident, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!($tile.$attr(), $expected);
            }
        };
    }

    create_test!(blank_svg_name, MahjongTile::BLANK, svg_name, "HONOUR_BLANK");
    create_test!(
        central_category,
        MahjongTile::CENTRAL,
        category,
        TileCategory::HONOUR(HonourCategory::DRAGON)
    );
    create_test!(
        central_svg_name,
        MahjongTile::CENTRAL,
        svg_name,
        "HONOUR_CENTRAL"
    );
    create_test!(central_value, MahjongTile::CENTRAL, value, None);
    create_test!(
        east_category,
        MahjongTile::EAST,
        category,
        TileCategory::HONOUR(HonourCategory::WIND)
    );
    create_test!(east_svg_name, MahjongTile::EAST, svg_name, "HONOUR_EAST");
    create_test!(east_value, MahjongTile::EAST, value, None);
    create_test!(
        prosperity_svg_name,
        MahjongTile::PROSPERITY,
        svg_name,
        "HONOUR_PROSPERITY"
    );
    create_test!(
        tiao5_category,
        MahjongTile::TIAO(5),
        category,
        TileCategory::TIAO
    );
    create_test!(tiao5_svg_name, MahjongTile::TIAO(5), svg_name, "TIAO_5");
    create_test!(tiao5_value, MahjongTile::TIAO(5), value, Some(5));
    create_test!(
        tong1_category,
        MahjongTile::TONG(1),
        category,
        TileCategory::TONG
    );
    create_test!(tong1_svg_name, MahjongTile::TONG(1), svg_name, "TONG_1");
    create_test!(tong1_value, MahjongTile::TONG(1), value, Some(1));
    create_test!(
        wan9_category,
        MahjongTile::WAN(9),
        category,
        TileCategory::WAN
    );
    create_test!(wan9_svg_name, MahjongTile::WAN(9), svg_name, "WAN_9");
    create_test!(wan9_value, MahjongTile::WAN(9), value, Some(9));
}

#[cfg(test)]
mod test_chinese_names {
    use super::*;
    macro_rules! create_test {
        ($name:ident, $tile:expr, $expected:literal) => {
            #[test]
            fn $name() {
                assert_eq!($tile.chinese_name(), $expected);
            }
        };
    }

    create_test!(blank_chinese_name, MahjongTile::BLANK, "白");
    create_test!(central_chinese_name, MahjongTile::CENTRAL, "中");
    create_test!(east_chinese_name, MahjongTile::EAST, "东");
    create_test!(prosperity_chinese_name, MahjongTile::PROSPERITY, "发");
    create_test!(tiao5_chinese_name, MahjongTile::TIAO(5), "五条");
    create_test!(tong1_chinese_name, MahjongTile::TONG(1), "一筒");
    create_test!(wan9_chinese_name, MahjongTile::WAN(9), "九万");
    create_test!(orchid_chinese_name, MahjongTile::ORCHID, "兰");
    create_test!(plum_chinese_name, MahjongTile::PLUM, "梅");
    create_test!(chrysanthemum_chinese_name, MahjongTile::CHRYSANTHEMUM, "菊");
    create_test!(bamboo_chinese_name, MahjongTile::BAMBOO, "竹");
}
