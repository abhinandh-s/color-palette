use color_palette::Palette;

#[derive(Debug, Palette)]
enum Mocha {
    #[color("#f5e0dc")] Rosewater,
    #[color("#1e1e2e")] Base,
    #[color("#cdd6f4")] Text,
}

#[test]
fn test_to_nix() {
}