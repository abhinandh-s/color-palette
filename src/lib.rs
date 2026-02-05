#[cfg(feature = "derive")]
pub use color_palette_macros::Palette;

pub trait Palette: std::fmt::Display {
    fn to_css() -> String;
}
