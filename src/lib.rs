use std::fmt::Display;
use std::num::ParseIntError;

pub trait Palette {
    #[cfg(feature = "css")]
    fn to_css() -> String;
}

// pub trait PaletteExt {
//     fn hex(color: &'static str) -> Hex {
//         Hex::from(color)
//     }
// }
//
// // Blanket impl for all palette structs
// impl<T> PaletteExt for T {}

#[cfg(feature = "derive")]
pub use color_palette_macros::palette;

#[cfg(feature = "derive")]
pub use color_palette_macros::Palette;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_hex(&self) -> Hex {
        Hex(format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b))
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

pub struct Hex(String);

impl Display for Hex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Hex {
    pub fn new(hex: &str) -> Self {
        Self(hex.to_owned())
    }

    pub fn to_rgb(&self) -> Result<Rgb, ColorError> {
        let hex = self.0.trim_start_matches('#');
        let n = u32::from_str_radix(hex, 16)?;

        match hex.len() {
            6 => Ok(Rgb {
                r: ((n >> 16) & 0xFF) as u8,
                g: ((n >> 8) & 0xFF) as u8,
                b: (n & 0xFF) as u8,
            }),
            3 => {
                let r = ((n >> 8) & 0xF) as u8;
                let g = ((n >> 4) & 0xF) as u8;
                let b = (n & 0xF) as u8;
                Ok(Rgb {
                    r: (r << 4) | r,
                    g: (g << 4) | g,
                    b: (b << 4) | b,
                })
            }
            _ => Err(ColorError::new("Invalid hex length")),
        }
    }
}

impl TryFrom<&'static str> for Hex {
    type Error = &'static str;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        match value.len() {
            3 | 6 => Ok(Self(value.to_owned())),
            _ => Err("Invalid hex length"),
        }
    }
}

impl TryFrom<String> for Hex {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.len() {
            3 | 6 => Ok(Self(value)),
            _ => Err("Invalid hex length"),
        }
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
