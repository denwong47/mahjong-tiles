use super::{FlowerCategory, HasChineseValue, HonourCategory, TileCategory};

/// A single Mahjong tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
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

impl PartialOrd for Tile {
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

impl Tile {
    /// Create a new Mahjong tile with a value.
    pub fn new_valued(category: TileCategory, value: u8) -> Self {
        match category {
            TileCategory::TONG => Tile::TONG(value),
            TileCategory::WAN => Tile::WAN(value),
            TileCategory::TIAO => Tile::TIAO(value),
            _ => unreachable!("Unreachable: Cannot create a non-number tile with a value"),
        }
    }
}

/// A trait for Mahjong tiles.
pub trait IsTile: PartialOrd + Eq + std::fmt::Debug + std::hash::Hash {
    /// Returns the Chinese name of the tile.
    fn unique_name(&self) -> String;

    /// Returns the category of the tile.
    fn category(&self) -> TileCategory;
    /// Returns the value of the tile, if it has one.
    fn value(&self) -> Option<u8>;

    /// Returns the direction of the tile, if it has one.
    /// This only applies to the Winds, Flowers and Seasons.
    fn direction(&self) -> Option<u8>;

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
        let category_name = self.category().category_name();

        if self.is_honour() {
            format!("{}-{}", category_name, format!("{:?}", self).to_lowercase())
        } else {
            format!(
                "{}-{}",
                category_name,
                self.value()
                    .expect("Unreachable: Number tiles should have a value")
            )
        }
    }
}

impl IsTile for Tile {
    /// Returns the chinese name for the tile.
    fn unique_name(&self) -> String {
        match self {
            Tile::EAST => "东".to_string(),
            Tile::SOUTH => "南".to_string(),
            Tile::WEST => "西".to_string(),
            Tile::NORTH => "北".to_string(),
            Tile::CENTRAL => "中".to_string(),
            Tile::PROSPERITY => "发".to_string(),
            Tile::BLANK => "白".to_string(),
            Tile::TIAO(v) | Tile::TONG(v) | Tile::WAN(v) => {
                let category_name = self.category().category_name_chinese();
                let chinese_value = v.chinese_value();
                format!("{}{}", chinese_value, category_name)
            }
            Tile::PLUM => "梅".to_string(),
            Tile::ORCHID => "兰".to_string(),
            Tile::CHRYSANTHEMUM => "菊".to_string(),
            Tile::BAMBOO => "竹".to_string(),
            Tile::SPRING => "春".to_string(),
            Tile::SUMMER => "夏".to_string(),
            Tile::AUTUMN => "秋".to_string(),
            Tile::WINTER => "冬".to_string(),
        }
    }

    fn category(&self) -> TileCategory {
        match self {
            Tile::EAST | Tile::SOUTH | Tile::WEST | Tile::NORTH => {
                TileCategory::HONOUR(HonourCategory::WIND)
            }
            Tile::CENTRAL | Tile::PROSPERITY | Tile::BLANK => {
                TileCategory::HONOUR(HonourCategory::DRAGON)
            }
            Tile::TONG(_) => TileCategory::TONG,
            Tile::WAN(_) => TileCategory::WAN,
            Tile::TIAO(_) => TileCategory::TIAO,
            Tile::PLUM | Tile::ORCHID | Tile::CHRYSANTHEMUM | Tile::BAMBOO => {
                TileCategory::FLOWER(FlowerCategory::FLOWER)
            }
            Tile::SPRING | Tile::SUMMER | Tile::AUTUMN | Tile::WINTER => {
                TileCategory::FLOWER(FlowerCategory::SEASON)
            }
        }
    }

    fn value(&self) -> Option<u8> {
        match self {
            Tile::TONG(v) | Tile::WAN(v) | Tile::TIAO(v) => Some(*v),
            _ => None,
        }
    }

    fn direction(&self) -> Option<u8> {
        match self {
            Tile::EAST | Tile::SPRING | Tile::PLUM => Some(1),
            Tile::SOUTH | Tile::SUMMER | Tile::ORCHID => Some(2),
            Tile::WEST | Tile::AUTUMN | Tile::CHRYSANTHEMUM => Some(3),
            Tile::NORTH | Tile::WINTER | Tile::BAMBOO => Some(4),
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

    create_test!(east_east, Tile::EAST, Tile::EAST, true);
    create_test!(east_south, Tile::EAST, Tile::SOUTH, false);
    create_test!(east_tong1, Tile::EAST, Tile::TONG(1), false);
    create_test!(tong1_tong1, Tile::TONG(1), Tile::TONG(1), true);
    create_test!(tong1_tong2, Tile::TONG(1), Tile::TONG(2), false);
    create_test!(tong1_wan1, Tile::TONG(1), Tile::WAN(1), false);
    create_test!(tong1_central, Tile::TONG(1), Tile::CENTRAL, false);
    create_test!(tong1_tiao1, Tile::TONG(1), Tile::TIAO(1), false);
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

    create_test!(east_cmp_east, Tile::EAST, Tile::EAST, None);
    create_test!(east_cmp_south, Tile::EAST, Tile::SOUTH, None);
    create_test!(east_lt_east, Tile::EAST < Tile::EAST, false);

    create_test!(east_le_east, Tile::EAST <= Tile::EAST, false);
    create_test!(east_gt_east, Tile::EAST > Tile::EAST, false);
    create_test!(east_ge_east, Tile::EAST >= Tile::EAST, false);
    create_test!(east_lt_south, Tile::EAST < Tile::SOUTH, false);
    create_test!(east_lt_central, Tile::EAST < Tile::CENTRAL, false);
    create_test!(east_lt_tong1, Tile::EAST < Tile::TONG(1), false);
    create_test!(east_lt_wan1, Tile::EAST < Tile::WAN(1), false);
    create_test!(east_lt_tiao1, Tile::EAST < Tile::TIAO(1), false);
    create_test!(central_cmp_central, Tile::CENTRAL, Tile::CENTRAL, None);
    create_test!(central_lt_central, Tile::CENTRAL < Tile::CENTRAL, false);
    create_test!(
        central_cmp_prosperity,
        Tile::CENTRAL,
        Tile::PROSPERITY,
        None
    );
    create_test!(
        central_lt_prosperity,
        Tile::CENTRAL < Tile::PROSPERITY,
        false
    );
    create_test!(
        tong1_cmp_tong1,
        Tile::TONG(1),
        Tile::TONG(1),
        Some(Ordering::Equal)
    );
    create_test!(tong1_lt_tong1, Tile::TONG(1) < Tile::TONG(1), false);
    create_test!(tong1_le_tong1, Tile::TONG(1) <= Tile::TONG(1), true);
    create_test!(
        tong1_cmp_tong2,
        Tile::TONG(1),
        Tile::TONG(2),
        Some(Ordering::Less)
    );
    create_test!(tong1_lt_tong2, Tile::TONG(1) < Tile::TONG(2), true);
    create_test!(tong1_cmp_wan1, Tile::TONG(1), Tile::WAN(1), None);
    create_test!(tong1_lt_wan1, Tile::TONG(1) < Tile::WAN(1), false);
    create_test!(tong1_cmp_central, Tile::TONG(1), Tile::CENTRAL, None);
    create_test!(tong1_lt_central, Tile::TONG(1) < Tile::CENTRAL, false);
    create_test!(tong1_cmp_tiao1, Tile::TONG(1), Tile::TIAO(1), None);
    create_test!(tong1_lt_tiao1, Tile::TONG(1) < Tile::TIAO(1), false);
    create_test!(
        tong9_cmp_tong1,
        Tile::TONG(9),
        Tile::TONG(1),
        Some(Ordering::Greater)
    );
    create_test!(tong9_lt_tong1, Tile::TONG(9) < Tile::TONG(1), false);
    create_test!(tong9_ge_tong1, Tile::TONG(9) >= Tile::TONG(1), true);
    create_test!(tong9_ge_wan1, Tile::TONG(9) >= Tile::WAN(1), false);
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

    create_test!(blank_svg_name, Tile::BLANK, svg_name, "honour-blank");
    create_test!(
        central_category,
        Tile::CENTRAL,
        category,
        TileCategory::HONOUR(HonourCategory::DRAGON)
    );
    create_test!(central_svg_name, Tile::CENTRAL, svg_name, "honour-central");
    create_test!(central_value, Tile::CENTRAL, value, None);
    create_test!(
        east_category,
        Tile::EAST,
        category,
        TileCategory::HONOUR(HonourCategory::WIND)
    );
    create_test!(east_svg_name, Tile::EAST, svg_name, "honour-east");
    create_test!(east_value, Tile::EAST, value, None);
    create_test!(
        prosperity_svg_name,
        Tile::PROSPERITY,
        svg_name,
        "honour-prosperity"
    );
    create_test!(tiao5_category, Tile::TIAO(5), category, TileCategory::TIAO);
    create_test!(tiao5_svg_name, Tile::TIAO(5), svg_name, "tiao-5");
    create_test!(tiao5_value, Tile::TIAO(5), value, Some(5));
    create_test!(tong1_category, Tile::TONG(1), category, TileCategory::TONG);
    create_test!(tong1_svg_name, Tile::TONG(1), svg_name, "tong-1");
    create_test!(tong1_value, Tile::TONG(1), value, Some(1));
    create_test!(wan9_category, Tile::WAN(9), category, TileCategory::WAN);
    create_test!(wan9_svg_name, Tile::WAN(9), svg_name, "wan-9");
    create_test!(wan9_value, Tile::WAN(9), value, Some(9));
}

#[cfg(test)]
mod test_chinese_names {
    use super::*;
    macro_rules! create_test {
        ($name:ident, $tile:expr, $expected:literal) => {
            #[test]
            fn $name() {
                assert_eq!($tile.unique_name(), $expected);
            }
        };
    }

    create_test!(blank_chinese_name, Tile::BLANK, "白");
    create_test!(central_chinese_name, Tile::CENTRAL, "中");
    create_test!(east_chinese_name, Tile::EAST, "东");
    create_test!(prosperity_chinese_name, Tile::PROSPERITY, "发");
    create_test!(tiao5_chinese_name, Tile::TIAO(5), "五条");
    create_test!(tong1_chinese_name, Tile::TONG(1), "一筒");
    create_test!(wan9_chinese_name, Tile::WAN(9), "九万");
    create_test!(orchid_chinese_name, Tile::ORCHID, "兰");
    create_test!(plum_chinese_name, Tile::PLUM, "梅");
    create_test!(chrysanthemum_chinese_name, Tile::CHRYSANTHEMUM, "菊");
    create_test!(bamboo_chinese_name, Tile::BAMBOO, "竹");
}
