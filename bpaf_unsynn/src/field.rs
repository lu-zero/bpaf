//! Field type analysis
//!
//! Re-exports TypeShape from parsing module for convenient field type analysis.

#[allow(unused_imports)]
pub use crate::parsing::TypeShape;

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use unsynn::{IParse, ToTokens};

    fn parse_type(s: &str) -> TypeShape {
        let ty: TokenStream = s.parse().unwrap();
        let mut iter = ty.to_token_iter();
        iter.parse::<TypeShape>().unwrap()
    }

    #[test]
    fn test_bool_type() {
        let shape = parse_type("bool");
        assert!(shape.is_bool());
    }

    #[test]
    fn test_unit_type() {
        let shape = parse_type("()");
        assert!(shape.is_unit());
    }

    #[test]
    fn test_option_type() {
        let shape = parse_type("Option<String>");
        assert!(shape.is_optional());
        assert_eq!(shape.inner_type().unwrap().to_string(), "String");
    }

    #[test]
    fn test_vec_type() {
        let shape = parse_type("Vec<i32>");
        assert!(shape.is_multiple());
        assert_eq!(shape.inner_type().unwrap().to_string(), "i32");
    }

    #[test]
    fn test_direct_type() {
        let shape = parse_type("String");
        assert!(matches!(shape, TypeShape::Direct(_)));
        assert_eq!(shape.inner_type().unwrap().to_string(), "String");
    }

    #[test]
    fn test_nested_option_vec() {
        let shape = parse_type("Option<Vec<String>>");
        assert!(shape.is_optional());
        assert_eq!(shape.inner_type().unwrap().to_string(), "Vec < String >");
    }

    #[test]
    fn test_nested_vec_option() {
        let shape = parse_type("Vec<Option<i32>>");
        assert!(shape.is_multiple());
        assert_eq!(shape.inner_type().unwrap().to_string(), "Option < i32 >");
    }

    #[test]
    fn test_complex_nested() {
        let shape = parse_type("Option<HashMap<String, Vec<u8>>>");
        assert!(shape.is_optional());
        assert_eq!(
            shape.inner_type().unwrap().to_string(),
            "HashMap < String , Vec < u8 > >"
        );
    }
}
