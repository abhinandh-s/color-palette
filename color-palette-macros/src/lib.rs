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
    variant.attrs.iter()
        .find(|a| a.path().is_ident("color"))
        .map(|attr| {
            attr.parse_args::<LitStr>()
                .expect("Expected #[color(\"#hex\")]")
                .value()
        })
        .unwrap_or_else(|| panic!("Missing #[color] for {}", variant.ident))
}
