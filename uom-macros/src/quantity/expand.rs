use super::*;
use proc_macro2::TokenStream;
use quote::quote;
// use syn::{AttrStyle, Token};

/// Expand the `quantity!` macro invocation to define the parsed quantity and associated units.
pub(crate) fn expand(input: Quantity) -> Result<TokenStream, syn::Error> {
    let Quantity {
        name_attributes,
        name,
        // description,
        // dimension_attributes,
        // system,
        // dimensions,
        // kind,
        // units,
        ..
    } = input;
    // let mod_comments = name_attributes
    //     .iter()
    //     .map(|a| {
    //         Attribute {
    //             style: AttrStyle::Inner(Token![!]([a.pound_token.span.clone()])),
    //             ..a.clone()
    //         }
    //     });

    let ts = quote! {
        #(#name_attributes)*
        pub struct #name;
    };

    Ok(ts)
}
