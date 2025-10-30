//! Core parsing primitives using unsynn
//!
//! This module defines basic building blocks for parsing Rust syntax

pub use unsynn::*;

// Re-export commonly used types
pub use proc_macro2::{Ident, TokenStream, TokenTree};

// Type alias for the iterator type we use
pub type TokenIter<'a> = unsynn::TokenIter<'a>;

// Keywords we'll need for parsing structs and enums
keyword! {
    pub KStruct = "struct";
    pub KEnum = "enum";
    pub KPub = "pub";
}

// Common operators
operator! {
    pub Comma = ",";
    pub Colon = ":";
    pub Semi = ";";
}
