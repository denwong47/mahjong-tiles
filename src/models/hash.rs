//! Implementation of hashing operations on [`Tile`].
//!

use crate::models::{FlowerCategory, HonourCategory, IsTile, Tile, TileCategory};

impl std::hash::Hash for TileCategory {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let id: u8 = match self {
            Self::HONOUR(HonourCategory::WIND) => 0b00,
            Self::HONOUR(HonourCategory::DRAGON) => 0b01,
            Self::FLOWER(FlowerCategory::FLOWER) => 0b10,
            Self::FLOWER(FlowerCategory::SEASON) => 0b11,
            Self::TONG => 0b100,
            Self::WAN => 0b101,
            Self::TIAO => 0b110,
        };

        state.write_u8(id);
    }
}

impl std::hash::Hash for Tile {
    /// Hash the tile into a unique identifier.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let category = self.category();
        category.hash(state);
        match &category {
            TileCategory::HONOUR(HonourCategory::WIND) | TileCategory::FLOWER(_) => state.write_u8(
                self.direction()
                    .expect("Wind, Flowers and Seasons should have a direction."),
            ),
            TileCategory::HONOUR(_) => match self {
                Self::CENTRAL => state.write_u8(0),
                Self::PROSPERITY => state.write_u8(1),
                Self::BLANK => state.write_u8(2),
                _ => unreachable!(),
            },
            _ => state.write_u8(
                self.value()
                    .expect("Non-value tile detected when one is expected."),
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::{hash_map::DefaultHasher, HashSet};
    use std::hash::{Hash, Hasher};

    macro_rules! create_test {
        ($name:ident($tile:expr) -> $expected:expr) => {
            #[test]
            fn $name() {
                let mut hasher = DefaultHasher::new();
                $tile.hash(&mut hasher);
                assert_eq!(hasher.finish(), $expected);
            }
        };
    }

    create_test!(east_hash(Tile::EAST) -> 75343234424780393);
    create_test!(tong_1_hash(Tile::TONG(1)) -> 9673533474134675344);
    create_test!(wan_9_hash(Tile::WAN(9)) -> 10510162281686547424);

    #[test]
    fn assert_unique_hashes() {
        let mut tiles = vec![
            Tile::EAST,
            Tile::SOUTH,
            Tile::WEST,
            Tile::NORTH,
            Tile::CENTRAL,
            Tile::PROSPERITY,
            Tile::BLANK,
            Tile::PLUM,
            Tile::ORCHID,
            Tile::CHRYSANTHEMUM,
            Tile::BAMBOO,
            Tile::SPRING,
            Tile::SUMMER,
            Tile::AUTUMN,
            Tile::WINTER,
        ];

        (1..=9).for_each(|i| {
            tiles.push(Tile::TONG(i));
            tiles.push(Tile::WAN(i));
            tiles.push(Tile::TIAO(i));
        });

        let hashes: HashSet<u64> = tiles
            .iter()
            .map(|t| {
                let mut hasher = DefaultHasher::new();
                t.hash(&mut hasher);
                hasher.finish()
            })
            .collect();

        assert_eq!(hashes.len(), tiles.len());
    }
}
