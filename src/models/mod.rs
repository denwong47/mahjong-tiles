//! Declaration of typical models used in Mahjong.

mod arithmetic;

mod category;
pub use category::*;

/// Re-export [`HasSvgData`] trait for use in other modules.
pub use crate::svg::HasSvgData;

mod tile;
pub use tile::*;

mod value;
pub(crate) use value::*;
