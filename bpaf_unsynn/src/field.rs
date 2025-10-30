//! Field type analysis

use proc_macro2::TokenStream;

/// The shape of a field's type - determines what kind of parser to generate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// bool type - generates switch()
    Bool,
    /// () unit type - generates pure(())
    Unit,
    /// Option<T> - generates optional()
    Option,
    /// Vec<T> - generates collect() with many()
    Vec,
    /// Any other type T - generates argument()
    Direct,
}

/// Analyze a type TokenStream to determine its shape
/// Returns (Shape, inner_type) where inner_type is T from Option<T> or Vec<T>
pub fn analyze_type(ty: &TokenStream) -> (Shape, TokenStream) {
    // Convert TokenStream to string for simple analysis
    // We'll parse it more carefully later, but this works for now
    let ty_str = ty.to_string();
    let ty_str = ty_str.trim();

    // Check for bool
    if ty_str == "bool" {
        return (Shape::Bool, TokenStream::new());
    }

    // Check for unit type
    if ty_str == "()" {
        return (Shape::Unit, TokenStream::new());
    }

    // Check for Option<T>
    if ty_str.starts_with("Option <") || ty_str.starts_with("Option<") {
        let inner = extract_generic_arg(ty_str, "Option");
        return (Shape::Option, inner);
    }

    // Check for Vec<T>
    if ty_str.starts_with("Vec <") || ty_str.starts_with("Vec<") {
        let inner = extract_generic_arg(ty_str, "Vec");
        return (Shape::Vec, inner);
    }

    // Default: Direct type
    (Shape::Direct, ty.clone())
}

/// Extract the generic argument from a type like "Option<T>" or "Vec<T>"
fn extract_generic_arg(ty_str: &str, _wrapper: &str) -> TokenStream {
    // Simple extraction: find < and > and take what's between
    if let Some(start) = ty_str.find('<') {
        if let Some(end) = ty_str.rfind('>') {
            let inner = &ty_str[start + 1..end].trim();
            // Parse the inner type back to TokenStream
            return inner.parse().unwrap_or_else(|_| TokenStream::new());
        }
    }
    TokenStream::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_type() {
        let ty: TokenStream = "bool".parse().unwrap();
        let (shape, _) = analyze_type(&ty);
        assert_eq!(shape, Shape::Bool);
    }

    #[test]
    fn test_unit_type() {
        let ty: TokenStream = "()".parse().unwrap();
        let (shape, _) = analyze_type(&ty);
        assert_eq!(shape, Shape::Unit);
    }

    #[test]
    fn test_option_type() {
        let ty: TokenStream = "Option<String>".parse().unwrap();
        let (shape, inner) = analyze_type(&ty);
        assert_eq!(shape, Shape::Option);
        assert_eq!(inner.to_string(), "String");
    }

    #[test]
    fn test_vec_type() {
        let ty: TokenStream = "Vec<i32>".parse().unwrap();
        let (shape, inner) = analyze_type(&ty);
        assert_eq!(shape, Shape::Vec);
        assert_eq!(inner.to_string(), "i32");
    }

    #[test]
    fn test_direct_type() {
        let ty: TokenStream = "String".parse().unwrap();
        let (shape, inner) = analyze_type(&ty);
        assert_eq!(shape, Shape::Direct);
        assert_eq!(inner.to_string(), "String");
    }
}
