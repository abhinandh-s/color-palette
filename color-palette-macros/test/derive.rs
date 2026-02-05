use color_palette::Palette;

#[derive(Debug, Palette)]
enum Latte {
    #[color("#dc8a78")] Rosewater,
    #[color("#dd7878")] Flamingo,
}

#[test]
fn test_css_output() {
    let css = Latte::to_css();
    assert!(css.contains("--color-latte-rosewater: #dc8a78;"));
}

#[test]
fn test_display() {
    assert_eq!(format!("{}", Latte::Flamingo), "#dd7878");
}
