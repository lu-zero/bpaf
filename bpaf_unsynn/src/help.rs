//! Help text representation
//!
//! This module provides the `Help` enum which can represent help text as either:
//! - A string literal extracted from doc comments (`Doc`)
//! - A custom expression like `format!("...")` or a constant (`Custom`)

use proc_macro2::TokenStream;
use quote::ToTokens;

/// Represents help text for parsers, options, and commands
///
/// Help text can come from two sources:
/// 1. Doc comments (`/// ...`) which become `Help::Doc`
/// 2. Explicit help expressions like `help(CONST)` which become `Help::Custom`
#[derive(Debug, Clone)]
pub enum Help {
    /// A custom expression provided by the user
    /// Examples: `help(MY_CONST)`, `help(format!("v{}", VERSION))`
    Custom(TokenStream),
    /// A string literal, typically from doc comments
    Doc(String),
}

impl ToTokens for Help {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Help::Custom(c) => c.to_tokens(tokens),
            Help::Doc(d) => d.to_tokens(tokens),
        }
    }
}

impl From<&str> for Help {
    fn from(value: &str) -> Self {
        Help::Doc(value.to_string())
    }
}

impl From<String> for Help {
    fn from(value: String) -> Self {
        Help::Doc(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_from_str() {
        let help: Help = "test string".into();
        match help {
            Help::Doc(s) => assert_eq!(s, "test string"),
            Help::Custom(_) => panic!("Expected Help::Doc"),
        }
    }

    #[test]
    fn test_help_from_string() {
        let help: Help = String::from("test string").into();
        match help {
            Help::Doc(s) => assert_eq!(s, "test string"),
            Help::Custom(_) => panic!("Expected Help::Doc"),
        }
    }

    #[test]
    fn test_help_from_empty_str() {
        let help: Help = "".into();
        match help {
            Help::Doc(s) => assert_eq!(s, ""),
            Help::Custom(_) => panic!("Expected Help::Doc"),
        }
    }
}
