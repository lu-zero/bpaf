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
use unsynn::{IParse, ToTokens};

/// Convert unsynn::Error to a compile_error! TokenStream with proper span
fn unsynn_error_to_compile_error(e: unsynn::Error) -> proc_macro2::TokenStream {
    let err = e.to_string();
    if let Some(token) = e.failed_at() {
        let span = token.span();
        quote::quote_spanned! { span => compile_error!(#err); }
    } else {
        quote::quote! { compile_error!(#err); }
    }
}

/// Derive macro for bpaf command line parser
///
/// For documentation refer to bpaf library: <https://docs.rs/bpaf/latest/bpaf/>
#[proc_macro_derive(Bpaf, attributes(bpaf))]
pub fn derive_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input2: proc_macro2::TokenStream = input.into();
    let mut iter = unsynn::ToTokens::to_token_iter(&input2);

    match iter.parse::<Top>() {
        Ok(top) => top.into_token_stream().into(),
        Err(e) => unsynn_error_to_compile_error(e).into(),
    }
}
