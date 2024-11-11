/// The sub-categories for Mahjong honours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HonourCategory {
    /// The four winds: East, South, West, North.
    WIND,
    /// The three dragons: Central, Prosperity, Blank.
    DRAGON,
}

/// The sub-categories for Flower tiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowerCategory {
    /// The four seasons: Spring, Summer, Autumn, Winter.
    SEASON,
    /// The four flowers: Plum, Orchid, Chrysanthemum, Bamboo.
    FLOWER,
}

/// The various categories of Mahjong tiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileCategory {
    /// The 4 seasons and 4 flowers.
    FLOWER(FlowerCategory),
    /// The 4 winds and 3 dragons.
    HONOUR(HonourCategory),
    /// The 9 circle tiles.
    TONG,
    /// The 9 ten thousand tiles.
    WAN,
    /// The 9 bamboo tiles.
    TIAO,
}

impl TileCategory {
    /// Get the name of the category in Chinese.
    pub fn category_name_chinese(&self) -> &'static str {
        match self {
            TileCategory::FLOWER(FlowerCategory::FLOWER) => "花",
            TileCategory::FLOWER(FlowerCategory::SEASON) => "季",
            TileCategory::HONOUR(HonourCategory::WIND) => "四风",
            TileCategory::HONOUR(HonourCategory::DRAGON) => "三元",
            TileCategory::TONG => "筒",
            TileCategory::WAN => "万",
            TileCategory::TIAO => "条",
        }
    }
    /// Get the name of the category.
    pub fn category_name(&self) -> &'static str {
        match self {
            TileCategory::FLOWER(FlowerCategory::FLOWER) => "flower",
            TileCategory::FLOWER(FlowerCategory::SEASON) => "season",
            TileCategory::HONOUR(_) => "honour",
            TileCategory::TONG => "tong",
            TileCategory::WAN => "wan",
            TileCategory::TIAO => "tiao",
        }
    }
}
