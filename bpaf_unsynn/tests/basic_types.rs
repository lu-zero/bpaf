//! Tests for basic type inference
//!
//! Covers: bool -> switch, String/numeric -> argument,
//! Option<T> -> optional, Vec<T> -> many, () -> pure,
//! and nested/complex generics.

use bpaf::Parser;

// =============================================================================
// Bool type -> switch
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct BoolField {
    /// A verbose flag
    verbose: bool,
}

#[test]
fn bool_becomes_switch() {
    let parser = BoolField::parse();

    // Without flag -> false
    let r = parser.run_inner(&[]).unwrap();
    assert!(!r.verbose);

    // With flag -> true
    let r = parser.run_inner(&["--verbose"]).unwrap();
    assert!(r.verbose);
}

// =============================================================================
// String/numeric types -> argument
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct StringField {
    name: String,
}

#[test]
fn string_becomes_argument() {
    let parser = StringField::parse();

    let r = parser.run_inner(&["--name", "Alice"]).unwrap();
    assert_eq!(r.name, "Alice");

    // Also works with =
    let r = parser.run_inner(&["--name=Bob"]).unwrap();
    assert_eq!(r.name, "Bob");
}

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct NumericFields {
    count: usize,
    value: i32,
    ratio: f64,
}

#[test]
fn numeric_becomes_argument() {
    let parser = NumericFields::parse();

    let r = parser
        .run_inner(&["--count", "42", "--value", "-10", "--ratio", "3.14"])
        .unwrap();
    assert_eq!(r.count, 42);
    assert_eq!(r.value, -10);
    assert!((r.ratio - 3.14).abs() < 0.001);
}

// =============================================================================
// Option<T> -> optional argument
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct OptionalField {
    name: Option<String>,
}

#[test]
fn option_becomes_optional() {
    let parser = OptionalField::parse();

    // Without -> None
    let r = parser.run_inner(&[]).unwrap();
    assert_eq!(r.name, None);

    // With value -> Some
    let r = parser.run_inner(&["--name", "Alice"]).unwrap();
    assert_eq!(r.name, Some("Alice".to_string()));
}

// =============================================================================
// Vec<T> -> many arguments
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct VecField {
    items: Vec<String>,
}

#[test]
fn vec_becomes_many() {
    let parser = VecField::parse();

    // No items -> empty vec
    let r = parser.run_inner(&[]).unwrap();
    assert!(r.items.is_empty());

    // Multiple items
    let r = parser
        .run_inner(&["--items", "a", "--items", "b", "--items", "c"])
        .unwrap();
    assert_eq!(r.items, vec!["a", "b", "c"]);
}

// =============================================================================
// () unit type -> pure(())
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct UnitField {
    #[bpaf(short('f'), long("flag"), req_flag(()))]
    flag: (),
    name: String,
}

#[test]
fn unit_with_req_flag() {
    let parser = UnitField::parse();

    let r = parser.run_inner(&["--flag", "--name", "test"]).unwrap();
    assert_eq!(r.flag, ());
    assert_eq!(r.name, "test");
}

// =============================================================================
// Nested generics - compilation tests
// These verify the parser handles complex generic types correctly
// For complex nested types, external parsers are recommended
// =============================================================================

/// Option<Vec<T>> with external parser - no implicit wrapping
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct NestedOptionVec {
    #[bpaf(external(parse_opt_vec), optional)]
    items: Option<Vec<String>>,
}

fn parse_opt_vec() -> impl Parser<Vec<String>> {
    bpaf::long("item")
        .argument::<String>("ITEM")
        .many()
}

#[test]
fn option_vec_nested() {
    // Verify it compiles and parses
    let parser = NestedOptionVec::parse().to_options();
    let r = parser.run_inner(&[]).unwrap();
    // Empty vec becomes Some([]) due to many() semantics
    assert_eq!(r.items, Some(vec![]));
}

/// Vec<Option<T>> with external parser
#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
struct NestedVecOption {
    #[bpaf(external(parse_optional_item), many)]
    items: Vec<Option<String>>,
}

fn parse_optional_item() -> impl Parser<Option<String>> {
    bpaf::long("item")
        .argument::<String>("ITEM")
        .optional()
}

#[test]
fn vec_option_nested() {
    // Verify it compiles
    let _ = NestedVecOption::parse();
}

// =============================================================================
// Complex generics (compilation test)
// =============================================================================

use std::collections::HashMap;

#[derive(Debug, Clone, bpaf_unsynn::Bpaf)]
#[allow(dead_code)]
struct ComplexGenerics {
    #[bpaf(external(parse_map))]
    map: HashMap<String, i32>,
}

fn parse_map() -> impl Parser<HashMap<String, i32>> {
    bpaf::pure(HashMap::new())
}

#[test]
fn complex_generics_compile() {
    // This compiles, which verifies complex generics in external work
    let _ = ComplexGenerics::parse();
}

// =============================================================================
// PathBuf and other stdlib types
// =============================================================================

use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct PathField {
    path: PathBuf,
}

#[test]
fn pathbuf_becomes_argument() {
    let parser = PathField::parse();

    let r = parser.run_inner(&["--path", "/tmp/test"]).unwrap();
    assert_eq!(r.path, PathBuf::from("/tmp/test"));
}

// =============================================================================
// Explicit bool type with long name (edge case for type inference)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct BoolWithLongName {
    #[bpaf(long("enable-feature"))]
    enabled: bool,
}

#[test]
fn bool_with_explicit_long_name() {
    let parser = BoolWithLongName::parse();

    let r = parser.run_inner(&[]).unwrap();
    assert!(!r.enabled);

    let r = parser.run_inner(&["--enable-feature"]).unwrap();
    assert!(r.enabled);
}

// =============================================================================
// Unit type with pure fallback (edge case)
// =============================================================================

#[derive(Debug, Clone, PartialEq, bpaf_unsynn::Bpaf)]
#[bpaf(options)]
struct UnitWithPure {
    #[bpaf(pure(()))]
    unit_field: (),
    name: String,
}

#[test]
fn unit_with_pure_attribute() {
    let parser = UnitWithPure::parse();

    let r = parser.run_inner(&["--name", "test"]).unwrap();
    assert_eq!(r.unit_field, ());
    assert_eq!(r.name, "test");
}
