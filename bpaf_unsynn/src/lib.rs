//! # bpaf_unsynn - Derive macros for bpaf using unsynn
//!
//! `bpaf_unsynn` provides derive macros for the [bpaf](https://docs.rs/bpaf) command line parser,
//! using the lightweight `unsynn` parser instead of `syn`. This is a drop-in replacement for
//! `bpaf_derive` with identical functionality but lighter dependencies.
//!
//! Use the derive macro:
//! ```no_run
//! use bpaf::Parser;
//! use bpaf_unsynn::Bpaf;
//!
//! #[derive(Debug, Clone, Bpaf)]
//! #[bpaf(options)]
//! struct Args {
//!     /// Verbose output
//!     #[bpaf(short, long)]
//!     verbose: bool,
//!
//!     /// Output file
//!     #[bpaf(short, long, argument("FILE"))]
//!     output: String,
//! }
//!
//! fn main() {
//!     println!("{:?}", args().run());
//! }
//! ```

mod attrs;
mod mode;
mod parsing;
mod top;
mod utils;

use top::Top;
use unsynn::{ErrorKind, IParse, ToTokens};

/// Convert unsynn::Error to a compile_error! TokenStream with proper span
fn unsynn_error_to_compile_error(e: unsynn::Error) -> proc_macro2::TokenStream {
    let err = match e.kind {
        ErrorKind::UnexpectedToken => {
            format!("Unexpected token: expecting {}", e.expected_type_name())
        }
        ErrorKind::Other { ref reason } => format!("Parser failed: {reason}"),
        _ => e.to_string(),
    };
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
    let mut iter = input2.to_token_iter();

    match iter.parse::<Top>() {
        Ok(top) => top.into_token_stream().into(),
        Err(e) => unsynn_error_to_compile_error(e).into(),
    }
}
