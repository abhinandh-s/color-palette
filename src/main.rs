use color_palette::{Hex, Rgb, palette};

fn main() {
    // #a6e3a1
    let rgb = color_palette::Hex::new("#a6e3a1").to_rgb().unwrap();
    println!("hex: #a6e3a1");
    assert_eq!(
        rgb,
        Rgb {
            r: 166,
            g: 227,
            b: 161
        }
    );
    println!("{}", rgb);
    println!("{:?}", rgb);
    println!("{:#?}", rgb);
    println!("rgb_to_hex: {}", rgb.to_hex());

    let teal = Hex::try_from(Mocha::GREEN).unwrap().to_rgb().unwrap();
    println!("teal: {}", teal);
}


palette! {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Mocha {
        rosewater: "#f5e0dc",
        flamingo: "#f2cdcd",
        pink: "#f5c2e7",
        mauve: "#cba6f7",
        red: "#f38ba8",
        maroon: "#eba0ac",
        peach: "#fab387",
        yellow: "#f9e2af",
        green: "#a6e3a1",
        teal: "#94e2d5",
        sky: "#89dceb",
        sapphire: "#74c7ec",
        blue: "#89b4fa",
        lavender: "#b4befe",
        text: "#cdd6f4",
        subtext1: "#bac2de",
        subtext0: "#a6adc8",
        overlay2: "#9399b2",
        overlay1: "#7f849c",
        overlay0: "#6c7086",
        surface2: "#585b70",
        surface1: "#45475a",
        surface0: "#313244",
        base: "#1e1e2e",
        mantle: "#181825",
        crust: "#11111b",
    }
}
