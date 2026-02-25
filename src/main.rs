use std::fmt::Display;
use std::num::ParseIntError;

fn main() {
    // #a6e3a1
    let rgb = hex_to_rgb("#FFF").unwrap();
    // assert_eq!(
    //     rgb,
    //     Rgb {
    //         r: 166,
    //         g: 227,
    //         b: 161
    //     }
    // );
    println!("{}", rgb);
    println!("{:?}", rgb);
    println!("{:#?}", rgb);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

fn hex_to_rgb(arg: &str) -> Result<Rgb, ColorError> {
    let hex = arg.trim_start_matches('#');
    let n = u32::from_str_radix(hex, 16).map_err(|_| ColorError::new("Invalid hex characters"))?;

    match hex.len() {
        6 => Ok(Rgb {
            r: ((n >> 16) & 0xFF) as u8,
            g: ((n >> 8) & 0xFF) as u8,
            b: (n & 0xFF) as u8,
        }),
        3 => Ok(Rgb {
            r: ((n >> 8) & 0xF) as u8,
            g: ((n >> 4) & 0xF) as u8,
            b: (n & 0xF) as u8,
        }),
        _ => Err(ColorError::new("Invalid hex length")),
    }
}

#[derive(Debug)]
pub struct ColorError(String);

impl ColorError {
    pub fn new(err: &str) -> Self {
        Self(err.to_owned())
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl From<ParseIntError> for ColorError {
    fn from(value: ParseIntError) -> Self {
        Self(value.to_string())
    }
}
