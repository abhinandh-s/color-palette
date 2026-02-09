pub trait Palette {
    #[cfg(feature = "css")]
    fn to_css() -> String;
}

#[cfg(feature = "derive")]
pub use color_palette_macros::palette;

#[cfg(feature = "derive")]
pub use color_palette_macros::Palette;
