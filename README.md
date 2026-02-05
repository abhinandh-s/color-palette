# color-palette

A type-safe color management system for Rust. Define your palettes once using enums and automatically generate CSS variables, Display implementations, and theme-switching logic.
Project Structure

This project is organized as a workspace to separate the core trait from the procedural macro logic:

- color-palette: The main crate containing the Palette trait and core types.
- color-palette-macros: The procedural macro crate that provides the `#[derive(Palette)]` functionality.

## Installation

Add this to your Cargo.toml:


```toml
[dependencies]
color-palette = { git = "https://github.com/abhinandh-s/color-palette", version = "0.1.0", features = ["derive"] }
```

## Usage

1. Define a Palette

Use the `#[derive(Palette)]` macro on an enum. Each variant represents a color, and the `#[color("#hex")]` attribute defines the value.

```Rust
use color_palette::Palette;

#[derive(Debug, Palette)]
enum Mocha {
    #[color("#f5e0dc")] Rosewater,
    #[color("#1e1e2e")] Base,
    #[color("#cdd6f4")] Text,
}
```

2. Generate CSS Variables

The Palette trait provides a to_css() method that creates a string of CSS variables formatted as `--color-<enum_name>-<variant_name>`.

```Rust
fn main() {
    println!("{}", Mocha::to_css());
}

/* Output:
  --color-mocha-rosewater: #f5e0dc;
  --color-mocha-base: #1e1e2e;
  --color-mocha-text: #cdd6f4;
*/
```

3. Display Trait

The macro automatically implements std::fmt::Display. Printing a variant will give you its hex code directly.

```Rust
fn main() {
    let color = Mocha::Rosewater;
    println!("The hex code is {}", color); // Output: The hex code is #f5e0dc
}
```
