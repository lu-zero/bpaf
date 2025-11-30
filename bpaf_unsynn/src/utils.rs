//! Utility functions
//!
//! This module contains helper utilities for the bpaf_unsynn derive macro.

/// Convert identifier to snake_case
pub fn to_snake_case(input: &str) -> String {
    to_custom_case(input, '_')
}

/// Convert identifier to kebab-case
pub fn to_kebab_case(input: &str) -> String {
    to_custom_case(input, '-')
}

/// Convert identifier to custom case with given separator
fn to_custom_case(input: &str, sep: char) -> String {
    let mut res = String::with_capacity(input.len() * 2);
    for c in input.strip_prefix("r#").unwrap_or(input).chars() {
        if c.is_ascii_uppercase() {
            if !res.is_empty() {
                res.push(sep);
            }
            res.push(c.to_ascii_lowercase());
        } else if c == '-' || c == '_' {
            res.push(sep);
        } else {
            res.push(c);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_to_snake_case() {
        assert_eq!(to_snake_case("Foo"), "foo");
        assert_eq!(to_snake_case("FooBar"), "foo_bar");
        assert_eq!(to_snake_case("FOO"), "f_o_o");
        assert_eq!(to_snake_case("r#in"), "in");
    }

    #[test]
    fn check_to_kebab_case() {
        assert_eq!(to_kebab_case("Foo"), "foo");
        assert_eq!(to_kebab_case("FooBar"), "foo-bar");
        assert_eq!(to_kebab_case("FOO"), "f-o-o");
        assert_eq!(to_kebab_case("r#in"), "in");
        assert_eq!(to_kebab_case("foo_bar"), "foo-bar");
    }
}
