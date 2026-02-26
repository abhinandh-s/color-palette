use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::Parse;
use syn::{
    Attribute, Data, DeriveInput, Ident, LitStr, Token, Visibility, braced, parse_macro_input,
};

/// Represents the entire palette! macro input
struct PaletteInput {
    schemes: Vec<SchemeDefinition>,
}
/// Represents a single color scheme definition
struct SchemeDefinition {
    attrs: Vec<Attribute>,
    vis: Visibility,
    name: Ident,
    colors: Vec<(Ident, LitStr)>,
}

impl Parse for PaletteInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut schemes = Vec::new();
        while !input.is_empty() {
            // Parse attributes like #[derve(Clone. Copy)]
            let attrs = input.call(Attribute::parse_outer)?;

            // Parse Visibility
            let vis: Visibility = input.parse()?;

            input.parse::<Token![struct]>()?;

            // Parse struct name
            let name: Ident = input.parse()?;

            // Parse the braced contents
            let contents;
            braced!(contents in input);

            let mut colors = Vec::new();
            while !contents.is_empty() {
                let color_name: Ident = contents.parse()?;
                contents.parse::<Token![:]>()?;
                let color_val: LitStr = contents.parse()?;

                // Optional trailing comma
                if contents.peek(Token![,]) {
                    contents.parse::<Token![,]>()?;
                }

                colors.push((color_name, color_val));
            }
            schemes.push(SchemeDefinition {
                attrs,
                vis,
                name,
                colors,
            });
        }
        Ok(PaletteInput { schemes })
    }
}

#[proc_macro]
pub fn palette(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as PaletteInput);
    let mut expanded = quote!();

    for scheme in input.schemes {
        let SchemeDefinition {
            attrs,
            vis,
            name,
            colors,
        } = scheme;

        // -- validating value
        for (id, val) in &colors {
            let raw = val.value();
            let hex = raw.trim_start_matches('#');

            if !raw.starts_with('#') {
                return syn::Error::new(
                    val.span(),
                    format!("color `{}`: value must start with '#', got {:?}", id, raw),
                )
                .to_compile_error()
                .into();
            }

            if hex.len() != 3 && hex.len() != 6 {
                return syn::Error::new(
                    val.span(),
                    format!(
                        "color `{}`: hex must be 3 or 6 characters after '#', got {} (\"{}\")",
                        id,
                        hex.len(),
                        raw
                    ),
                )
                .to_compile_error()
                .into();
            }

            if !hex.chars().all(|c| c.is_ascii_hexdigit()) {
                return syn::Error::new(val.span(), format!(
                    "color `{}`: invalid hex characters in {:?}",
                    id, raw
                ))
                .to_compile_error()
                .into();
            }
        }

        // --
        let consts = colors.iter().map(|(id, val)| {
            let const_name = format_ident!("{}", id.to_string().to_uppercase());
            quote! {
                #vis const #const_name: &'static str = #val;
            }
        });

        let name_l = name.to_string().to_lowercase();

        #[allow(unused_variables)] // if css feature is off it will warn unused_variables
        let css_body = colors.iter().map(|(id, val)| {
            let css_line = format!("    --color-{}-{}: {};\n", name_l, id, val.value());
            quote! {
                s.push_str(#css_line);
            }
        });

        expanded.extend(quote! {
            #(#attrs)*
            #vis struct #name;

            impl #name {
                #(#consts)*
            }
        });

        #[cfg(feature = "css")]
        expanded.extend(quote! {
            impl color_palette::Palette for #name {
                fn to_css() -> String {
                    let mut s = String::new();
                    #(#css_body)*
                    s
                }
            }
        });
    }

    TokenStream::from(expanded)
}

// for Enum
#[proc_macro_derive(Palette, attributes(color))]
pub fn derive_color_theme(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let theme_name_str = name.to_string().to_lowercase();

    let variants = if let Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        panic!("Palette derive can only be used on enums");
    };

    let display_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let hex_value = get_hex_attr(variant);
        quote! { Self::#variant_ident => write!(f, #hex_value) }
    });

    // We check the feature flag of the macro crate itself during expansion
    let css_impl = if cfg!(feature = "css") {
        let css_generators = variants.iter().map(|variant| {
            let variant_ident = &variant.ident;
            let css_name = variant_ident.to_string().to_lowercase().replace("_", "-");
            let hex_value = get_hex_attr(variant);

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

        quote! {
            impl color_palette::Palette for #name {
                fn to_css() -> String {
                    let mut ctx = String::new();
                    #(#css_generators)*
                    ctx
                }
            }
        }
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms),*
                }
            }
        }
        #css_impl
    };

    TokenStream::from(expanded)
}

fn get_hex_attr(variant: &syn::Variant) -> String {
    variant
        .attrs
        .iter()
        .find(|a| a.path().is_ident("color"))
        .map(|attr| {
            attr.parse_args::<LitStr>()
                .expect("Expected #[color(\"#hex\")]")
                .value()
        })
        .unwrap_or_else(|| panic!("Missing #[color] for {}", variant.ident))
}
