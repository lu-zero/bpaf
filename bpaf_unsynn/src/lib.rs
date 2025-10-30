//! # Derive macro for bpaf command line parser (using unsynn)
//!
//! This is an alternative implementation of `bpaf_derive` that uses `unsynn` instead of `syn`.
//! For documentation refer to `bpaf` library docs: <https://docs.rs/bpaf/latest/bpaf/>

mod attrs;
mod field;
mod parsing;
mod top;

use top::Top;
use unsynn::IParse;

/// Derive macro for bpaf command line parser
///
/// For documentation refer to bpaf library: <https://docs.rs/bpaf/latest/bpaf/>
#[proc_macro_derive(Bpaf, attributes(bpaf))]
pub fn derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Convert to proc_macro2::TokenStream and create token iterator
    let input2: proc_macro2::TokenStream = input.into();
    let mut iter = unsynn::ToTokens::to_token_iter(&input2);

    // Parse the input using unsynn
    match iter.parse::<Top>() {
        Ok(top) => {
            // Convert the parsed structure back to tokens
            quote::quote! { #top }.into()
        }
        Err(e) => {
            // Generate a compile error
            let err = format!("Parse error: {:?}", e);
            quote::quote! {
                compile_error!(#err);
            }
            .into()
        }
    }
}
