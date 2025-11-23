//! # Derive macro for bpaf command line parser (using unsynn)
//!
//! This is an alternative implementation of `bpaf_derive` that uses `unsynn` instead of `syn`.
//! For documentation refer to `bpaf` library docs: <https://docs.rs/bpaf/latest/bpaf/>

mod attrs;
mod field;
mod help;
mod mode;
mod parsing;
mod top;
mod utils;

use top::Top;
use unsynn::IParse;

/// Derive macro for bpaf command line parser
///
/// For documentation refer to bpaf library: <https://docs.rs/bpaf/latest/bpaf/>
#[proc_macro_derive(Bpaf, attributes(bpaf))]
pub fn derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input2: proc_macro2::TokenStream = input.into();
    let mut iter = unsynn::ToTokens::to_token_iter(&input2);

    match iter.parse::<Top>() {
        Ok(top) => quote::quote! { #top }.into(),
        Err(e) => {
            let err = e.to_string();
            // Try to get the span from the failed token for better error location
            if let Some(token) = e.failed_at() {
                let span = token.span();
                quote::quote_spanned! { span =>
                    compile_error!(#err);
                }
                .into()
            } else {
                quote::quote! {
                    compile_error!(#err);
                }
                .into()
            }
        }
    }
}
