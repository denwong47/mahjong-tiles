//! Build time generated from `{{SOURCE_PATH}}`.
//!

{{DECLARATIONS}}

/// Get the SVG data for the given name.
///
/// # Returns
///
/// - `Some(&'static str)`: The SVG data for the given name.
/// - `None`: If the name is not found.
pub fn get_svg_by_name(name: &str) -> Option<&'static str> {
    match name {
{{MATCHES}}
        _ => None,
    }
}
