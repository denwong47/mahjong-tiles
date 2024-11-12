//! Rust bindings for a Mahjong set, with SVG images.
//!
//! # Example
//!
//! ```rust
//! use mahjong_tiles::{Tile, TileCategory, IsTile, HonourCategory, HasSvgData, SvgStyle};
//!
//! let tile = Tile::TONG(1);
//!
//! assert_eq!(tile.category(), TileCategory::TONG);
//! assert_eq!(tile.value(), Some(1));
//! assert_eq!(tile.chinese_name(), "一筒");
//! assert_eq!(tile + 1, Some(Tile::TONG(2)));
//!
//! let svg = tile.svg_data(mahjong_tiles::SvgStyle::DARK).unwrap();
//! assert!(svg.starts_with("data:image/svg+xml;base64,"));
//!
//! let tile = Tile::WAN(9);
//!
//! assert_eq!(tile.category(), TileCategory::WAN);
//! assert_eq!(tile.value(), Some(9));
//! assert_eq!(tile.chinese_name(), "九万");
//! assert_eq!(tile - 1, Some(Tile::WAN(8)));
//!
//! let tile = Tile::EAST;
//!
//! assert_eq!(tile.category(), TileCategory::HONOUR(HonourCategory::WIND));
//! ```

mod models;
mod svg;

pub use models::*;
pub use svg::SvgStyle;

#[doc(hidden)]
pub mod validator;
