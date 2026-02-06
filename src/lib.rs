pub trait Palette {
    // If the feature is off, the trait won't have this method
    #[cfg(feature = "css")]
    fn to_css() -> String;
}

#[cfg(feature = "derive")]
pub use color_palette_macros::Palette;
