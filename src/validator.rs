#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{FlowerCategory, HonourCategory, IsTile, Tile, TileCategory};

/// A collection of tiles grouped by category.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GroupedTiles<'t> {
    pub winds: Vec<&'t Tile>,
    pub dragons: Vec<&'t Tile>,
    pub tongs: Vec<&'t Tile>,
    pub tiaos: Vec<&'t Tile>,
    pub wans: Vec<&'t Tile>,
    pub flowers: Vec<&'t Tile>,
    pub seasons: Vec<&'t Tile>,
}

impl<'t> GroupedTiles<'t> {
    /// Get the total number of tiles in the grouped tiles.
    pub fn len(&self) -> usize {
        self.winds.len()
            + self.dragons.len()
            + self.tongs.len()
            + self.tiaos.len()
            + self.wans.len()
            + self.flowers.len()
            + self.seasons.len()
    }

    /// Get a hash set of all the tiles in the grouped tiles.
    pub fn hash_set(&self) -> std::collections::HashSet<&&Tile> {
        std::collections::HashSet::from_iter(
            self.winds
                .iter()
                .chain(self.dragons.iter())
                .chain(self.tongs.iter())
                .chain(self.wans.iter())
                .chain(self.tiaos.iter())
                .chain(self.flowers.iter())
                .chain(self.seasons.iter()),
        )
    }
}

impl<'t> FromIterator<&'t Tile> for GroupedTiles<'t> {
    /// Create a new `GroupedTiles` from an iterator of tiles.
    ///
    /// This enables the following syntax:
    ///
    /// ```rust
    /// use mahjong_tiles::{validator::GroupedTiles, Tile};
    ///
    /// let tiles = vec![
    ///     Tile::EAST,
    ///     Tile::SOUTH,
    ///     Tile::TONG(1),
    ///     Tile::TIAO(5),
    ///     Tile::WAN(9),
    /// ];
    ///
    /// let grouped_tiles: GroupedTiles = tiles.iter().collect();
    ///
    /// assert_eq!(grouped_tiles.winds.len(), 2);
    /// assert_eq!(grouped_tiles.tongs.len(), 1);
    /// ```
    fn from_iter<T: IntoIterator<Item = &'t Tile>>(iter: T) -> Self {
        iter.into_iter().fold(Self::default(), |mut acc, tile| {
            match tile.category() {
                TileCategory::HONOUR(HonourCategory::WIND) => acc.winds.push(tile),
                TileCategory::HONOUR(HonourCategory::DRAGON) => acc.dragons.push(tile),
                TileCategory::TONG => acc.tongs.push(tile),
                TileCategory::TIAO => acc.tiaos.push(tile),
                TileCategory::WAN => acc.wans.push(tile),
                TileCategory::FLOWER(FlowerCategory::FLOWER) => acc.flowers.push(tile),
                TileCategory::FLOWER(FlowerCategory::SEASON) => acc.seasons.push(tile),
            }
            acc
        })
    }
}

pub struct MahjongHand<'t> {
    closed: GroupedTiles<'t>,
    open: Vec<Meld<'t>>,
}

impl MahjongHand<'_> {
    pub fn is_complete(&self) -> bool {
        self.closed.len() == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuadStatus {
    Open,
    Closed,
}

pub enum Meld<'t> {
    Sequence([&'t Tile; 3]),
    Triplet([&'t Tile; 3]),
    Quad([&'t Tile; 4], QuadStatus),
    // Unsupported.
    // SkipBy4([&'t Tile; 3]),
}

pub struct Eye<'t>(&'t Tile);

#[non_exhaustive]
pub enum MahjongCombination<'t> {
    Melds([Meld<'t>; 4], Eye<'t>),
    ThirteenOrphans(Eye<'t>),
    // Unsupported.
    // AllNonMelds([Meld<'t>; 4], Eye<'t>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MahjongScore {
    ThirteenOrphans,
    MajorFourWinds,
    MajorThreeDragons,
    MinorFourWinds,
    MinorThreeDragons,
    Pure(TileCategory),
    AllTriplets,
    AllSequences,
    AllQuads, // AKA 18 Arhats

    MatchingSeason,
    MatchingFlower,
    AllConcealed,
    SelfDrawn,
    LastTile, // AKA Moon From the Bottom of the Sea
    RobbingTheKong,
}

pub struct PossibleCombinationsIterator<'t> {
    tiles: &'t MahjongHand<'t>,
    index: usize,
}

/// An iterator to move one meld at a time from closed tiles to open tiles,
/// resolving the hand into a set of melds and an eye in the process.
impl<'t> Iterator for PossibleCombinationsIterator<'t> {
    type Item = MahjongHand<'t>;

    fn next(&mut self) -> Option<MahjongHand<'t>> {
        unimplemented!("Not yet implemented")
    }
}

pub fn is_13_orphans<'t>(tiles: &'t GroupedTiles) -> Option<MahjongCombination<'t>> {
    if tiles.len() != 14 {
        return None;
    }

    let required = [
        &&Tile::TIAO(1),
        &&Tile::TIAO(9),
        &&Tile::WAN(1),
        &&Tile::WAN(9),
        &&Tile::TONG(1),
        &&Tile::TONG(9),
        &&Tile::EAST,
        &&Tile::SOUTH,
        &&Tile::WEST,
        &&Tile::NORTH,
        &&Tile::CENTRAL,
        &&Tile::PROSPERITY,
        &&Tile::BLANK,
    ];

    let required_hash = std::collections::HashSet::from(required);
    let actual_hash = tiles.hash_set();

    if actual_hash == required_hash {
        unimplemented!()
    } else {
        None
    }
}

pub fn iter_possible_combinations<'t>(hand: &'t MahjongHand) -> PossibleCombinationsIterator<'t> {
    unimplemented!("Not yet implemented")
}

pub fn score_melds(melds: &[Meld], eye: &Eye) -> Vec<MahjongScore> {
    unimplemented!("Not yet implemented")
}

pub fn find_highest_scoring_hand<'t>(hand: &'t MahjongHand<'t>) -> Option<MahjongCombination<'t>> {
    if let Some(combo) = is_13_orphans(&hand.closed) {
        // If you have a 13 Orphans combo, then you win immediately.
        return Some(combo);
    }
    unimplemented!("Not yet implemented")
}
