use crate::models::IsMahjongTile;

{{MODULES}}

/// Style for SVG elements.
pub enum SvgStyle {
{{STYLES}}
}

/// SVG data getting mapped to a style.
impl SvgStyle {
    /// Get the SVG data for a tile.
    pub fn get_svg_by_name(&self, name: &str) -> Option<&'static str> {
        match self {
{{MATCH}}
        }
    }
}

/// Trait for tiles that have SVG data.
pub trait HasSvgData: IsMahjongTile {
    /// Get the SVG data for this tile.
    fn svg_data(&self, style: SvgStyle) -> Option<&'static str>;
}

/// Add method to retrieve SVG data to all structs that implement [`IsMahjongTile`].
impl<T> HasSvgData for T where T: IsMahjongTile {
    /// Get the SVG data for this tile.
    fn svg_data(&self, style: SvgStyle) -> Option<&'static str> {
        style.get_svg_by_name(&self.svg_name())
    }
}
