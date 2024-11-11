//! Rust bindings for a Mahjong set, with SVG images.
//!
//! # Example
//!
//! ```rust
//! use mahjong::{MahjongTile, TileCategory, IsMahjongTile, HonourCategory, HasSvgData, SvgStyle};
//!
//! let tile = MahjongTile::TONG(1);
//!
//! assert_eq!(tile.category(), TileCategory::TONG);
//! assert_eq!(tile.value(), Some(1));
//! assert_eq!(tile.chinese_name(), "一筒");
//! assert_eq!(tile + 1, Some(MahjongTile::TONG(2)));
//!
//! let svg = tile.svg_data(mahjong::SvgStyle::DARK).unwrap();
//! assert!(svg.starts_with("data:image/svg+xml;base64,"));
//!
//! let tile = MahjongTile::WAN(9);
//!
//! assert_eq!(tile.category(), TileCategory::WAN);
//! assert_eq!(tile.value(), Some(9));
//! assert_eq!(tile.chinese_name(), "九万");
//! assert_eq!(tile - 1, Some(MahjongTile::WAN(8)));
//!
//! let tile = MahjongTile::EAST;
//!
//! assert_eq!(tile.category(), TileCategory::HONOUR(HonourCategory::WIND));
//! ```

mod models;
mod svg;

pub use models::*;
pub use svg::SvgStyle;
