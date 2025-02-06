extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, LitInt, LitStr, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct PacketArgs {
    packet_id: u8,
    packet_name: String,
}

impl Parse for PacketArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Expect a literal integer
        let packet_id_lit: LitInt = input.parse()?;
        // Parse the comma separator
        input.parse::<Token![,]>()?;
        // Expect a literal string
        let packet_name_lit: LitStr = input.parse()?;
        Ok(PacketArgs {
            packet_id: packet_id_lit.base10_parse::<u8>()?,
            packet_name: packet_name_lit.value(),
        })
    }
}

/// The macro attribute expansion.
/// It takes the two arguments and the struct item,
/// then implements PacketId for that struct.
pub fn expand_packet_id(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as PacketArgs);
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;
    let packet_id = args.packet_id;

    let packet_name = LitStr::new(&args.packet_name, proc_macro2::Span::call_site());
    let expanded = quote! {
        #[allow(dead_code)]
        #input

        impl PacketId for #struct_name {
            const PACKET_ID: u8 = #packet_id;
            const PACKET_NAME: &'static str = #packet_name;

            fn get_packet_id(&self) -> u8 {
                Self::PACKET_ID
            }

            fn get_packet_name(&self) -> &'static str {
                Self::PACKET_NAME
            }
        }
    };

    TokenStream::from(expanded)
}
