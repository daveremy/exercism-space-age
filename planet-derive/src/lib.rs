extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;

struct PlanetInput {
    pub planet_name: syn::Ident,
    pub seconds: syn::LitFloat,
}

impl Parse for PlanetInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let planet_name = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let seconds = input.parse()?;
        Ok(PlanetInput {
            planet_name,
            seconds,
        })
    }
}
#[proc_macro]
pub fn planet(input: TokenStream) -> TokenStream {
    let PlanetInput {
        planet_name,
        seconds,
    } = parse_macro_input!(input as PlanetInput);

    let expanded = quote! {
        pub struct #planet_name;
        impl Planet for #planet_name {
            const SECS: f64 = #seconds;
        }
    };

    TokenStream::from(expanded)
}
