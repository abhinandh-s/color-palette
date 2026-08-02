
use color_palette::Palette;
use color_palette::Palette as PaletteTrait;

#[derive(Palette)]
pub enum BaseTheme {
    #[color("#1a1a1a")]
    Background,
    #[color("#ff5555")]
    Error,
}

#[cfg(feature = "css")]
#[test]
fn test_css_generation() {
    let css = BaseTheme::to_css();
    let expected = "\
  --color-basetheme-background: #1a1a1a;
  --color-basetheme-error: #ff5555;
";
    assert_eq!(css, expected);
}

#[cfg(feature = "nix")]
#[test]
fn test_nix_generation() {
    let nix = BaseTheme::to_nix();
    let expected = "\
basetheme = {
    background= #1a1a1a;
    error= #ff5555;
};
";
    assert_eq!(nix, expected);
}
