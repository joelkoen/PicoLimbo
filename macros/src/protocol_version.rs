use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitInt, LitStr};

pub fn expand_protocol_version_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Get the identifier of the enum (e.g. ProtocolVersion)
    let enum_ident = input.ident;

    // Ensure the input is an enum.
    let data_enum = match input.data {
        syn::Data::Enum(data) => data,
        _ => {
            return syn::Error::new_spanned(
                enum_ident,
                "ProtocolVersion can only be derived for enums",
            )
            .to_compile_error()
            .into()
        }
    };

    // Prepare vectors for the match arms for each generated implementation.
    let mut display_arms = Vec::new();
    let mut humanize_arms = Vec::new();
    let mut from_i32_arms = Vec::new();
    let mut version_number_arms = Vec::new();
    let mut pvn_values = Vec::new();

    // Iterate over all variants of the enum.
    for variant in data_enum.variants.iter() {
        let variant_ident = &variant.ident;

        // Look for the #[pvn(...)] attribute.
        let mut pvn_value: Option<LitInt> = None;
        for attr in &variant.attrs {
            if attr.path().is_ident("pvn") {
                // Expect the attribute to be in the form: #[pvn(769)]
                pvn_value = Some(match attr.parse_args::<LitInt>() {
                    Ok(lit) => lit,
                    Err(err) => return err.to_compile_error().into(),
                });
                break;
            }
        }
        let pvn_value = match pvn_value {
            Some(val) => val,
            None => {
                return syn::Error::new_spanned(
                    variant,
                    "Missing #[pvn(...)] attribute on variant",
                )
                .to_compile_error()
                .into();
            }
        };

        // Parse the integer literal to an i32.
        let pvn_int = match pvn_value.base10_parse::<i32>() {
            Ok(num) => num,
            Err(err) => return err.to_compile_error().into(),
        };
        pvn_values.push((pvn_int, variant_ident));

        // For Display, we want to output the variant name as a string.
        display_arms.push(quote! {
            #enum_ident::#variant_ident => f.write_str(stringify!(#variant_ident))
        });

        // For From<i32>, map the provided integer to the corresponding variant.
        from_i32_arms.push(quote! {
            #pvn_value => #enum_ident::#variant_ident
        });

        // For version_number, we return the literal integer.
        version_number_arms.push(quote! {
            #enum_ident::#variant_ident => #pvn_value
        });

        let variant_name = variant_ident.to_string();
        let humanized = variant_name
            .strip_prefix('V')
            .unwrap_or(variant_name.as_str())
            .replace('_', ".");
        let humanized_lit = LitStr::new(&humanized, variant_ident.span());
        humanize_arms.push(quote! {
            #enum_ident::#variant_ident => #humanized_lit
        });
    }

    // Determine the smallest and largest pvn values and their associated variants.
    let (min_value, min_variant_ident) = pvn_values
        .iter()
        .min_by_key(|(val, _)| *val)
        .map(|(v, ident)| (v, ident))
        .expect("At least one variant is required");
    let (max_value, max_variant_ident) = pvn_values
        .iter()
        .max_by_key(|(val, _)| *val)
        .map(|(v, ident)| (v, ident))
        .expect("At least one variant is required");

    let expanded = quote! {
        impl std::fmt::Display for #enum_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms),*
                }
            }
        }

        impl From<i32> for #enum_ident {
            fn from(value: i32) -> Self {
                match value {
                    #(#from_i32_arms),*,
                    v if v > #max_value => #enum_ident::#max_variant_ident,
                    v if v < #min_value => #enum_ident::#min_variant_ident,
                    _ => Self::default(),
                }
            }
        }

        impl #enum_ident {
            pub fn version_number(&self) -> u32 {
                match self {
                    #(#version_number_arms),*
                }
            }

            pub fn humanize(&self) -> &'static str {
                match self {
                    #(#humanize_arms),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
