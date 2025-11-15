//! Utility functions
//!
//! This module contains helper utilities for the bpaf_unsynn derive macro.

/// Convert identifier to kebab-case
/// Handles both snake_case and camelCase
pub fn to_kebab_case(s: &str) -> String {
    let mut result = String::new();
    for (i, ch) in s.chars().enumerate() {
        if ch == '_' {
            result.push('-');
        } else if ch.is_uppercase() {
            if i > 0 {
                result.push('-');
            }
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }
    result
}
