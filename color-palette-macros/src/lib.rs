use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, LitStr, parse_macro_input};

#[proc_macro_derive(Palette, attributes(color))]
pub fn derive_color_theme(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let theme_name_str = name.to_string().to_lowercase();

    let variants = if let Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        panic!("ColorTheme can only be used on enums");
    };

    // Generate the CSS variable strings
    let css_generators = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let css_name = variant_ident.to_string().to_lowercase().replace("_", "-");

        let hex_value = variant
            .attrs
            .iter()
            .find(|a| a.path().is_ident("color"))
            .map(|attr| {
                attr.parse_args::<LitStr>()
                    .expect("Expected a string literal in #[color(\"#hex\")]")
                    .value()
            })
            .unwrap_or_else(|| panic!("Missing #[color(\"#hex\")] for variant {}", variant_ident));

        quote! {
            ctx.push_str("  --color-");
            ctx.push_str(#theme_name_str);
            ctx.push_str("-");
            ctx.push_str(#css_name);
            ctx.push_str(": ");
            ctx.push_str(#hex_value);
            ctx.push_str(";\n");
        }
    });

    // Generate match arms for the Display implementation
    let display_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let hex_value = variant
            .attrs
            .iter()
            .find(|a| a.path().is_ident("color"))
            .map(|attr| {
                attr.parse_args::<LitStr>()
                    .expect("Invalid attribute")
                    .value()
            })
            .unwrap();

        quote! {
            Self::#variant_ident => write!(f, #hex_value)
        }
    });

    let expanded = quote! {
        impl #name {
            /// Generates a string of CSS variables for this color theme.
            pub fn to_css() -> String {
                let mut ctx = String::new();
                #(#css_generators)*
                ctx
            }
        }

        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
