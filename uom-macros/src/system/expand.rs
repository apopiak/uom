use super::*;
use proc_macro2::TokenStream;
use quote::quote;
// use syn::{AttrStyle, Token};

/// Expand the `system!` macro invocation to define the system of quantities and default system of
/// units.
pub(crate) fn expand(input: System) -> Result<TokenStream, syn::Error> {
    let System {
        name_attributes,
        name,
        // units_attributes,
        // units,
        // base_quantities,
        quantities,
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
    let mods = quantities.iter().filter_map(|q| {
        if q.add_mod {
            Some(quote! { mod #(q.module); })
        } else {
            None
        }
    });

    let ts = quote! {
        // #(#mod_comments)*

        #(#name_attributes)*
        pub struct #name;

        #(#mods)*
    };

    Ok(ts)
}
