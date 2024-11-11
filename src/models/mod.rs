//! Declaration of typical models used in Mahjong.

mod category;
pub use category::*;

mod tile;
pub use tile::*;

/// Re-export [`HasSvgData`] trait for use in other modules.
pub use crate::svg::HasSvgData;

mod value;
pub(crate) use value::*;
